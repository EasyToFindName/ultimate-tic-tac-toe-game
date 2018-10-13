use actix::prelude::*;
use actix_web::{ws, ws::Message};
use std::time::{Duration, Instant};
use serde_json;

use game_lobby::TurnData;
use game_lobby::GameLobby;
use AppState;

use std::sync::Arc;

static CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
static PING_INTERVAL: Duration = Duration::from_secs(1);

pub struct GameSocket {
    pong_time: Instant,
    lobby_addr: Arc<Addr<GameLobby>>
}

impl GameSocket {
    pub fn new(lobby_addr: Arc<Addr<GameLobby>>) -> GameSocket {
        GameSocket {
            pong_time: Instant::now(), lobby_addr
        }
    }

    pub fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(PING_INTERVAL, |socket_actor, ctx| {
            let delta_time = Instant::now().duration_since(socket_actor.pong_time);

            if delta_time > CONNECTION_TIMEOUT {
                println!("Connection timed-out. Dropping web socket...");
                ctx.stop();
            }
            else {
                ctx.ping("");
            }
        });
    }
}


impl Actor for GameSocket {
    type Context = ws::WebsocketContext<Self, AppState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Web socket is opened");
        self.heartbeat(ctx);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for GameSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            Message::Pong(_any) => {
                self.pong_time = Instant::now();
            }
            Message::Text(text) => {
                let turn_data: TurnData = match serde_json::from_str(&text) {
                    Ok(data) => data,
                    Err(_) => {
                        println!("Warning: Suspicious input received: {}", text);
                        return;
                    }
                };

                println!("Got TurnData: {:?}", turn_data);
                self.lobby_addr.do_send(turn_data);
            }

            Message::Close(_any) => {
                println!("Web socket was closed");
            }
            _ => ()
        }
    }
}