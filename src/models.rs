use crate::schema::{players, games, moves, protocols, games_players};
use chrono::{NaiveDateTime, Utc};
use diesel::MysqlConnection;

pub struct MyNaiveDateTime(NaiveDateTime);

impl Default for MyNaiveDateTime {
    fn default() -> Self {
        MyNaiveDateTime(Utc::now().naive_utc())
    }
}

#[derive(Default, Queryable, Associations)]
#[belongs_to(Game)]
#[table_name = "protocols"]
pub struct Protocol {
    game_id: usize,
    protocol_number: usize,
    initiator_knowledge: String,
    responder_knowledge: String
}

#[derive(Queryable, Associations)]
#[table_name = "players"]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub token: String,
    pub lastseen_at: NaiveDateTime
}

impl Player {
    pub fn get(connection: &MysqlConnection, player_name: String) -> Option<Self> {
        use crate::schema::players::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::TextExpressionMethods;
        let result = players.filter(name.like(player_name)).first::<Player>(connection);
        result.ok()
    }
    pub fn insert(connection: &MysqlConnection, new_id: u64, new_name: String) -> Option<u64> {
        use crate::schema::players::dsl::*;
        use diesel::{insert_into};
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        //let new_id = rand::random::<u64>();
        let result = insert_into(players).values((id.eq(new_id), name.eq(new_name), token.eq("".to_string()), lastseen_at.eq(Utc::now().naive_utc()))).execute(connection);
        match result {
            Ok(_) => Some(new_id),
            Err(_) => None
        }
    }
    pub fn remove(connection: &MysqlConnection, the_id: u64) -> bool {
        use crate::schema::players::dsl::*;
        use diesel::delete;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;
        let result = delete(players.filter(id.eq(the_id))).execute(connection);
        match result {
            Ok(_) => true,
            Err(_) => false
        }
    }
}

#[derive(Default, Queryable, Associations)]
#[belongs_to(Game)]
#[belongs_to(Player)]
#[table_name = "moves"]
pub struct Move {
    game_id: usize,
    protocol_number: usize,
    move_number: usize,
    player_id: usize,
    protocol_message: String,
    new_knowledge: String
}

#[derive(Default, Queryable, Associations)]
#[table_name = "games"]
pub struct Game {
    id: usize,
    created_at: MyNaiveDateTime,
    finished_at: Option<MyNaiveDateTime>,
    creator_id: usize,
    initiator: String,
    responder: String,
    initial_knowledge_initiator: String,
    initial_knowledge_responder: String,
    task: String,
    joining_code: String,
    round: usize,
    rounds: usize,
    started: bool,
    finished: bool
}

#[derive(Default, Queryable, Associations)]
#[belongs_to(Player)]
#[belongs_to(Game)]
#[table_name = "games_players"]
pub struct GamePlayer {
    game_id: usize,
    player_id: usize,
    position: usize
}