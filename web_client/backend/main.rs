extern crate actix;
extern crate actix_web;

mod game_socket;

use actix_web::{
    fs, ws, App, Error, server,
    http::{header, Method},
    HttpRequest, HttpResponse,
};
use game_socket::GameSocket;


const SERVER_PORT: u16 = 3000;
const FRONTEND_FOLDER_PATH: &'static str = "web_client/frontend";

fn open_web_socket(request: &HttpRequest) -> Result<HttpResponse, Error> {
    ws::start(request, GameSocket::new())
}

fn redirect_to_main_page(_request: &HttpRequest) -> HttpResponse {
    HttpResponse::Found()
        .header(header::LOCATION, "/static/game.html")
        .finish()
}

fn main() {
    let server_addr = format!("localhost:{}", SERVER_PORT);
    println!("Starting up server on {}...", server_addr);

    server::new(|| {
        App::new()
            .handler(
                "/static",
                fs::StaticFiles::new(FRONTEND_FOLDER_PATH).unwrap(),
            )
            .resource("/ws", |r| r.method(Method::GET).f(open_web_socket))
            .resource("/", |r| r.method(Method::GET).f(redirect_to_main_page))

    }).bind(server_addr.clone())
      .expect(&format!("Cannot bind to {}", server_addr))
      .shutdown_timeout(0)
      .run();
}