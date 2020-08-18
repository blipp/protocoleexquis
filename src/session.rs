use log::{debug, info};
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Result};

use actix::fut;
use actix::prelude::*;
use actix_broker::BrokerIssue;
use actix_web_actors::ws;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::message::{GameMessage, GameMessageWrapper, CreateGame, ChatMessage, JoinRoom, DisconnectPlayer, ListRooms, SendMessage, SendIndivMessage, AddConnectedPlayer};
use crate::server::GameServer;
use crate::models::{Player};

use crate::db::{MysqlPool, mysql_pool_handler};

type Client = Addr<WsSession>;
#[derive(Default)]
pub struct State {
    // These are the players that currently have a Websocket connection
    // _and_ have already chosen a name (and thus are “logged in”).
    pub players: Mutex<HashMap<u64, Client>>
}

pub struct WsSession {
    state: web::Data<State>,
    pool: web::Data<MysqlPool>,
    player_id: Option<u64>,
    room: String,
    name: Option<String>,
}


impl WsSession {
    pub fn new(state: web::Data<State>,
        pool: web::Data<MysqlPool>,
        player_id: Option<u64>,
        room: String,
        name: Option<String>
    ) -> Self {
        WsSession {
            state: state,
            pool: pool,
            player_id: player_id,
            room: room,
            name: name
        }
    }
    pub fn add_to_players(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        if let Some(player_id) = self.player_id {
            let msg = AddConnectedPlayer(player_id, ctx.address().recipient());
            println!("just before issue sync for AddConnectedPlayer");
            GameServer::from_registry().send(msg).into_actor(self).then(|_, _, _| {fut::ready(())}).wait(ctx);
            //self.issue_system_sync(msg, ctx);
            println!("just after issue sync for AddConnectedPlayer");
        }
    }
    pub fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name = room_name.to_owned();

        /*if let Some(player_id) = self.player_id {
            // First send a leave message for the current room
            let leave_msg = LeaveRoom(self.room.clone(), player_id);
            // issue_sync comes from having the `BrokerIssue` trait in scope.
            self.issue_system_sync(leave_msg, ctx);
        }*/

        // Then send a join message for the new room
        let join_msg = JoinRoom(
            room_name.to_owned(),
            self.name.clone(),
            ctx.address().recipient(),
        );

        GameServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(id) = id {
                    act.room = room_name;
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn list_rooms(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        GameServer::from_registry()
            .send(ListRooms)
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Ok(rooms) = res {
                    for room in rooms {
                        ctx.text(room);
                    }
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn send_msg(&self, msg: &str) {
        if let Some(player_id) = self.player_id {
            let content = format!(
                "{}: {}",
                self.name.clone().unwrap_or_else(|| "anon".to_string()),
                msg
            );

            println!("{}", format!("in send_msg for '{}'", content));
            let msg = SendMessage(self.room.clone(), player_id, content);

            // issue_async comes from having the `BrokerIssue` trait in scope.
            self.issue_system_async(msg);
            println!("end send_msg");
        } else {
            println!("no message without a name!");
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        //self.join_room("Main", ctx);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!(
            "WsSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            match self.player_id {Some(id) => id.to_string(), None => "_".to_string()},
            self.room
        );
        if let Some(player_id) = self.player_id {
            let mysql_pool = mysql_pool_handler(self.pool.clone()).ok();
            if let Some(the_pool) = mysql_pool {
                let success = Player::remove(&the_pool, player_id);
            }
            let msg = DisconnectPlayer(player_id);
            self.issue_system_sync(msg, ctx);

            let mut players = self.state.players.lock().unwrap();
            players.remove(&player_id);
        }
    }
}

impl Handler<ChatMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        debug!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();

                if msg.starts_with('/') {
                    let mut command = msg.splitn(2, ' ');

                    match command.next() {
                        Some("/list") => self.list_rooms(ctx),

                        Some("/join") => {
                            if let Some(room_name) = command.next() {
                                self.join_room(room_name, ctx);
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }

                        Some("/name") => {
                            if let Some(name) = command.next() {
                                // first we look if a player with this name already exists
                                let mysql_pool = mysql_pool_handler(self.pool.clone()).ok();
                                if let Some(the_pool) = mysql_pool {
                                    let other_player = Player::get(&the_pool, name.to_string());

                                    // There is already another player with this name
                                    if let Some(player) = other_player {
                                        ctx.text(format!("{{\"type\": \"name\", \"msg\": \"fail\"}}"));
                                        // Send a message to this other player
                                        let msg = SendIndivMessage(player.id, "someone tried to get your name".to_string());
                                        self.issue_system_async(msg);
                                    // The name is not taken
                                    } else {
                                        // Assign an ID
                                        let new_player_id = rand::random::<u64>();
                                        let result = Player::insert(&the_pool, new_player_id, name.to_string());

                                        if let Some(new_player_id) = result {
                                            self.player_id = Some(new_player_id);
                                            println!("just before add_to_players");
                                            self.add_to_players(ctx);
                                            //let mut players = self.state.players.lock().unwrap();
                                            //players.insert(new_player_id, ctx.address());
                                        }

                                        self.name = Some(name.to_owned());
                                        ctx.text(format!("{{\"type\": \"name\", \"msg\": \"success\"}}"));
                                    }
                                }

                           } else {
                                ctx.text("!!! name is required");
                            }
                        }

                        _ => ctx.text(format!("!!! unknown command: {:?}", msg)),
                    }

                    return;
                }
                // try json
                let result = serde_json::from_str(msg);
                match result {
                    Ok(obj) =>
                        match obj {
                            GameMessageWrapper { msg_type, msg } =>
                                match msg {
                                    GameMessage::CreateGame(crt_game) => 
                                    //{ initiator, responder, task, rounds, initial_knowledge_initiator, initial_knowledge_responder } =>
                                        println!("{}", format!("{}, {}", crt_game.initiator, crt_game.responder)),
                                    _ => println!("other game message")
                                },
                            _ => println!("other json")
                        },
                    _ => {
                        println!("error or other msg");
                        self.send_msg(msg);
                    }

                }
                //let create_game_msg: GameMessage<CreateGame> = serde_json::from_str(msg)?;

            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
