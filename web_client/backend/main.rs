extern crate actix;
extern crate actix_web;
extern crate futures;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate env_logger;

#[macro_use]
extern crate log;

extern crate tic_tac_toe;

mod game_socket;
mod game_lobby;
mod messages;

use actix::prelude::*;
use actix_web::{
    fs, ws, App, Error, server,
    http::{header, Method},
    HttpRequest, HttpResponse, HttpMessage, AsyncResponder,
    middleware,
};

use serde_json::json;


use futures::{Future, Stream};
use game_socket::GameSocket;
use game_lobby::{GameLobby, GameConfiguration};
use std::{collections::HashMap, sync::Arc};

const SERVER_PORT: u16 = 3000;
const FRONTEND_FOLDER_PATH: &str = "web_client/frontend";


fn open_web_socket(request: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(request, GameSocket::new(Arc::clone(&request.state().lobby_addr)))
}

fn redirect_to_main_page(_request: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Found()
        .header(header::LOCATION, "/static/index.html")
        .finish()
}

fn create_lobby(request: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    request.json().from_err().and_then(|config: GameConfiguration| {
        info!("Requested lobby configuration: {:?}", config);

        match config.validate() {
            Ok(_) => Ok(HttpResponse::Ok().json(json!({"ok": "/static/game.html"}))),
            Err(e) => Ok(HttpResponse::Ok().json(json!({"error": e})))
        }

    }).responder()
}

pub struct AppState {
    pub lobby_addr: Arc<Addr<GameLobby>>,
}

fn main() {
    let server_addr = format!("localhost:{}", SERVER_PORT);

    std::env::set_var("RUST_LOG", "actix_web=debug,tic_tac_toe_server=debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let system = System::new("Game lobby");
    let lobby = Arc::new(GameLobby::new(2).start());

    server::new(move || {
        App::with_state(AppState{lobby_addr: Arc::clone(&lobby)})
            .middleware(middleware::Logger::default())
            .handler(
                "/static",
                fs::StaticFiles::new(FRONTEND_FOLDER_PATH).unwrap(),
            )
            .resource("/lobby/create", |r| r.method(Method::POST).f(create_lobby))
            .resource("/ws", |r| r.method(Method::GET).f(open_web_socket))
            .resource("/", |r| r.method(Method::GET).f(redirect_to_main_page))

    }).bind(server_addr.clone())
      .unwrap_or_else(|_| panic!("Cannot bind to {}", server_addr))
      .shutdown_timeout(0)
      .run();

    system.run();
}