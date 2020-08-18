use actix::prelude::*;
use actix_broker::BrokerSubscribe;

use std::collections::HashMap;
use std::mem;

use crate::message::{ChatMessage, JoinRoom, DisconnectPlayer, ListRooms, SendMessage, SendIndivMessage, AddConnectedPlayer};
use crate::models::{Player};
use crate::session::WsSession;

type Client = Recipient<ChatMessage>;
type Room = HashMap<u64, Client>;

#[derive(Default)]
pub struct GameServer {
    rooms: HashMap<String, Room>,
    players: HashMap<u64, Client>
}

impl GameServer {
    fn take_room(&mut self, room_name: &str) -> Option<Room> {
        let room = self.rooms.get_mut(room_name)?;
        let room = mem::replace(room, HashMap::new());
        Some(room)
    }

    fn add_client_to_room(
        &mut self,
        room_name: &str,
        id: Option<u64>,
        client: Client,
    ) -> u64 {
        let mut id = id.unwrap_or_else(rand::random::<u64>);

        if let Some(room) = self.rooms.get_mut(room_name) {
            loop {
                if room.contains_key(&id) {
                    id = rand::random::<u64>();
                } else {
                    break;
                }
            }

            room.insert(id, client);
            return id;
        }

        // Create a new room for the first client
        let mut room: Room = HashMap::new();

        room.insert(id, client);
        self.rooms.insert(room_name.to_owned(), room);

        id
    }

    fn send_chat_message(
        &mut self,
        room_name: &str,
        msg: &str,
        _src: u64,
    ) -> Option<()> {
/*        let mut room = self.take_room(room_name)?;

        for (id, client) in room.drain() {
            if client.do_send(ChatMessage(msg.to_owned())).is_ok() {
                self.add_client_to_room(room_name, Some(id), client);
            }
        }
*/
        println!("in send_chat_message");
        for (id, client) in self.players.iter() {
            println!("{}", format!("id {}", id));
            client.do_send(ChatMessage(msg.to_owned())).is_ok();
        }
        println!("end send_chat_message");

        Some(())
    }

    fn send_individual_message(
        &mut self,
        msg: &str,
        target: u64,
    ) -> Option<()> {

        let client = self.players.get(&target)?;
        client.do_send(ChatMessage(msg.to_owned())).is_ok();

        Some(())
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<DisconnectPlayer>(ctx);
        self.subscribe_system_async::<SendMessage>(ctx);
        self.subscribe_system_async::<SendIndivMessage>(ctx);
    }
}

impl Handler<JoinRoom> for GameServer {
    type Result = MessageResult<JoinRoom>;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom(room_name, client_name, client) = msg;

        let id = self.add_client_to_room(&room_name, None, client);
        let join_msg = format!(
            "{} joined {}",
            client_name.unwrap_or_else(|| "anon".to_string()),
            room_name
        );

        self.send_chat_message(&room_name, &join_msg, id);
        MessageResult(id)
    }
}

impl Handler<AddConnectedPlayer> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: AddConnectedPlayer, _ctx: &mut Self::Context) {
        let AddConnectedPlayer(player_id, client) = msg;
        println!("{}", format!("in handle AddConnectedPlayer {}", player_id.to_string()));
        self.players.insert(player_id, client);
        println!("{}", format!("self.players.length: {}", self.players.len().to_string()));
        println!("end of handle AddConnectedPlayer");
    }
}

impl Handler<DisconnectPlayer> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectPlayer, _ctx: &mut Self::Context) {
        self.players.remove(&msg.0);
        println!("{}", format!("self.players.length: {}", self.players.len().to_string()));
    }
}

impl Handler<ListRooms> for GameServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.rooms.keys().cloned().collect())
    }
}

impl Handler<SendMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) {
        let SendMessage(room_name, id, msg) = msg;
        self.send_chat_message(&room_name, &msg, id);
    }
}

impl Handler<SendIndivMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: SendIndivMessage, _ctx: &mut Self::Context) {
        let SendIndivMessage(id, msg) = msg;
        self.send_individual_message(&msg, id);
    }
}

impl SystemService for GameServer {}
impl Supervised for GameServer {}
