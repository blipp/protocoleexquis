CREATE TABLE players (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    token VARCHAR(255) NOT NULL,
    lastseen_at DATETIME NOT NULL
);

CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    created_at DATETIME NOT NULL,
    finished_at DATETIME,
    creator_id BIGINT UNSIGNED NOT NULL,
    initiator VARCHAR(255) NOT NULL DEFAULT 'Initiator',
    responder VARCHAR(255) NOT NULL DEFAULT 'Responder',
    initial_knowledge_initiator VARCHAR(1000) DEFAULT '',
    initial_knowledge_responder VARCHAR(1000) DEFAULT '',
    task VARCHAR(1000) DEFAULT '',
    joining_code VARCHAR(255) NOT NULL,
    round TINYINT UNSIGNED DEFAULT 0,
    rounds TINYINT UNSIGNED NOT NULL,
    started BOOLEAN DEFAULT false,
    finished BOOLEAN DEFAULT false,
    FOREIGN KEY (creator_id) REFERENCES players(id)
);

CREATE TABLE games_players (
    game_id BIGINT UNSIGNED NOT NULL,
    player_id BIGINT UNSIGNED NOT NULL,
    position TINYINT UNSIGNED NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games(id),
    FOREIGN KEY (player_id) REFERENCES players(id),
    PRIMARY KEY (game_id, player_id),
    UNIQUE (game_id, position)
);

CREATE TABLE protocols (
    game_id BIGINT UNSIGNED NOT NULL,
    protocol_number TINYINT UNSIGNED NOT NULL,
    initiator_knowledge VARCHAR(1000) DEFAULT '',
    responder_knowledge VARCHAR(1000) DEFAULT '',
    PRIMARY KEY (game_id, protocol_number),
    FOREIGN KEY (game_id) REFERENCES games(id),
    UNIQUE (game_id, protocol_number)
);

CREATE TABLE moves (
    game_id BIGINT UNSIGNED NOT NULL,
    protocol_number TINYINT UNSIGNED NOT NULL,
    move_number TINYINT UNSIGNED NOT NULL,
    player_id BIGINT UNSIGNED NOT NULL,
    protocol_message VARCHAR(1000),
    new_knowledge VARCHAR(1000),
    PRIMARY KEY (game_id, protocol_number, move_number),
    FOREIGN KEY (game_id) REFERENCES games(id),
    FOREIGN KEY (player_id) REFERENCES players(id)
);