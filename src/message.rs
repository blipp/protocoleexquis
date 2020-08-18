use actix::prelude::*;
use crate::session::WsSession;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "u64")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<ChatMessage>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
//pub struct AddConnectedPlayer(pub u64, pub Addr<WsSession>);
pub struct AddConnectedPlayer(pub u64, pub Recipient<ChatMessage>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct DisconnectPlayer(pub u64);

#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
pub struct ListRooms;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String, pub u64, pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendIndivMessage(pub u64, pub String);

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateGame {
    pub initiator: String,
    pub responder: String,
    pub task: String,
    pub rounds: usize,
    pub initial_knowledge_initiator: String,
    pub initial_knowledge_responder: String
}

#[derive(Clone, Deserialize, Serialize)]
pub enum GameMessage {
    CreateGame(CreateGame),
    JoinGame,
    SendMove
}

#[derive(Clone, Deserialize, Serialize)]
pub struct GameMessageWrapper {
    msg_type: String,
    msg: GameMessage
}