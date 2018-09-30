use actix::prelude::*;
use actix_web::ws;
use std::time::{Duration, Instant};

static CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
static PING_INTERVAL: Duration = Duration::from_secs(1);


pub struct GameSocket {
    pong_time: Instant,
}

impl GameSocket {
    pub fn new() -> GameSocket {
        GameSocket {
            pong_time: Instant::now(),
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
                println!("Time elapsed since pong: {:?}", delta_time);
                ctx.ping("");
            }
        });
    }
}


impl Actor for GameSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Web socket is opened");
        self.heartbeat(ctx);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for GameSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        use ws::Message;
        match msg {
            Message::Pong(_any) => {
                println!("Got pong message");
                self.pong_time = Instant::now();
            }
            _ => ()
        }
    }
}