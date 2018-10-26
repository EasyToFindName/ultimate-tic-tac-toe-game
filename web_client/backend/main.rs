extern crate actix;
extern crate actix_web;
extern crate futures;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate tic_tac_toe;

mod game_socket;
mod game_lobby;
mod messages;

use actix::prelude::*;
use actix_web::{
    fs, ws, App, Error, server,
    http::{header, Method},
    HttpRequest, HttpResponse,
};

use game_socket::GameSocket;
use game_lobby::GameLobby;
use std::sync::Arc;

const SERVER_PORT: u16 = 3000;
const FRONTEND_FOLDER_PATH: &str = "web_client/frontend";


fn open_web_socket(request: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(request, GameSocket::new(Arc::clone(&request.state().lobby_addr)))
}

fn redirect_to_main_page(_request: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Found()
        .header(header::LOCATION, "/static/game.html")
        .finish()
}

pub struct AppState {
    pub lobby_addr: Arc<Addr<GameLobby>>,
}

fn main() {
    let server_addr = format!("localhost:{}", SERVER_PORT);
    println!("Starting up server on {}...", server_addr);

    let system = System::new("Game lobby");
    let lobby = Arc::new(GameLobby::new(2).start());

    server::new(move || {
        App::with_state(AppState{lobby_addr: Arc::clone(&lobby)})
            .handler(
                "/static",
                fs::StaticFiles::new(FRONTEND_FOLDER_PATH).unwrap(),
            )
            .resource("/ws", |r| r.method(Method::GET).f(open_web_socket))
            .resource("/", |r| r.method(Method::GET).f(redirect_to_main_page))

    }).bind(server_addr.clone())
      .unwrap_or_else(|_| panic!("Cannot bind to {}", server_addr))
      .shutdown_timeout(0)
      .run();

    system.run();
}