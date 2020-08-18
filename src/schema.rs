table! {
    games (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        finished_at -> Nullable<Datetime>,
        creator_id -> Unsigned<Bigint>,
        initiator -> Varchar,
        responder -> Varchar,
        initial_knowledge_initiator -> Nullable<Varchar>,
        initial_knowledge_responder -> Nullable<Varchar>,
        task -> Nullable<Varchar>,
        joining_code -> Varchar,
        round -> Nullable<Unsigned<Tinyint>>,
        rounds -> Unsigned<Tinyint>,
        started -> Nullable<Bool>,
        finished -> Nullable<Bool>,
    }
}

table! {
    games_players (game_id, player_id) {
        game_id -> Unsigned<Bigint>,
        player_id -> Unsigned<Bigint>,
        position -> Unsigned<Tinyint>,
    }
}

table! {
    moves (game_id, protocol_number, move_number) {
        game_id -> Unsigned<Bigint>,
        protocol_number -> Unsigned<Tinyint>,
        move_number -> Unsigned<Tinyint>,
        player_id -> Unsigned<Bigint>,
        protocol_message -> Nullable<Varchar>,
        new_knowledge -> Nullable<Varchar>,
    }
}

table! {
    players (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        token -> Varchar,
        lastseen_at -> Datetime,
    }
}

table! {
    protocols (game_id, protocol_number) {
        game_id -> Unsigned<Bigint>,
        protocol_number -> Unsigned<Tinyint>,
        initiator_knowledge -> Nullable<Varchar>,
        responder_knowledge -> Nullable<Varchar>,
    }
}

joinable!(games -> players (creator_id));
joinable!(games_players -> games (game_id));
joinable!(games_players -> players (player_id));
joinable!(moves -> games (game_id));
joinable!(moves -> players (player_id));
joinable!(protocols -> games (game_id));

allow_tables_to_appear_in_same_query!(
    games,
    games_players,
    moves,
    players,
    protocols,
);
