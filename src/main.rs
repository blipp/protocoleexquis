#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

mod db;
mod message;
mod server;
mod session;
mod models;
mod schema;

use dotenv::dotenv;
use std::env;

use log::info;

use actix_files::Files;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix::prelude::*;
use diesel::prelude::*;

use db::{MysqlPool, connect};
use session::{State, WsSession};

use models::{Player};

async fn chat_route(
    state: web::Data<State>,
    pool: web::Data<MysqlPool>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(WsSession::new(
        state,
        pool,
        None,
        "".to_string(),
        None
    ), &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let addr = "127.0.0.1:34787";

    let srv = HttpServer::new(move || { // TODO what does `move` do? other examples do not have it here.
        let state = State::default();
        App::new()
            .data(state)
            .data(connect(dotenv!("DATABASE_URL")))
            .service(web::resource("/ws/").to(chat_route))
            .service(Files::new("/", "./static/").index_file("game.html"))
    })
    .bind(&addr)?;

    info!("Starting http server: {}", &addr);

    srv.run().await
}
