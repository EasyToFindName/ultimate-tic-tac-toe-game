use actix::prelude::*;
use actix_web::{ws, ws::Message, ws::WsWriter};
use std::time::{Duration, Instant};
use serde_json;

use game_lobby::GameLobby;
use messages::*;

use futures::future::Future;
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

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(PING_INTERVAL, |socket_actor, ctx| {
            let delta_time = Instant::now().duration_since(socket_actor.pong_time);

            if delta_time > CONNECTION_TIMEOUT {
                info!("Connection timed-out. Dropping web socket...");
                ctx.stop();
            }
            else {
                ctx.ping("");
            }
        });
    }

    fn send_message(&self, msg: &ClientMessage, ctx: &mut <Self as Actor>::Context) {
        let json_object = match serde_json::to_string(msg) {
            Ok(obj) => obj,
            Err(msg) => {
                error!("Bad message {:?}", msg);
                return;
            }
        };

        ctx.send_text(json_object);
    }
}


impl Actor for GameSocket {
    type Context = ws::WebsocketContext<Self, AppState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let is_registered = match self.lobby_addr
            .send(RegisterPlayer(ctx.address().clone()))
            .wait() {
                Ok(st) => st,
                Err(msg) => {
                    error!("{}", msg);
                    ctx.send_close(None);
                    ctx.stop();
                    return;
                }
            };

        if is_registered {
            info!("Connected to the lobby");
            self.heartbeat(ctx);

        } else {
            self.send_message(&ClientMessage::Info(String::from("The lobby is full!")), ctx);
            info!("The lobby if full. Closing GameSocket...");
            ctx.send_close(None);
            ctx.stop();
            return;
        }
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("GameSocket was stopped");
        self.lobby_addr.do_send(PlayerDisconnected(ctx.address()));
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for GameSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            Message::Pong(_any) => {
                self.pong_time = Instant::now();
            }
            Message::Text(text) => {
                let turn_data: Position = match serde_json::from_str(&text) {
                    Ok(data) => data,
                    Err(_) => {
                        warn!("Suspicious input received: `{}`", text);
                        return;
                    }
                };

                debug!("Got TurnData: {:?}", turn_data);
                self.lobby_addr.do_send(MakeTurn{player_addr: ctx.address(), turn_data});
            }

            Message::Close(_any) => {
                info!("GameSocket was closed");
            }
            _ => ()
        }
    }
}

impl Handler<ClientMessage> for GameSocket {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        self.send_message(&msg, ctx);
    }
}

impl Handler<LobbyClosed> for GameSocket {
    type Result = ();

    fn handle(&mut self, _msg: LobbyClosed, ctx: &mut Self::Context) {
        debug!("GameSocket: Recevied Lobby closed event");
        ctx.send_close(None);
        ctx.stop();
    }
}