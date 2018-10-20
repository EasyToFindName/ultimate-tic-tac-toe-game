use actix::prelude::*;
use serde_json;
use game_socket::GameSocket;

// helper struct
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessage {
    Cross(Position),
    Circle(Position),
    Line(Position, Position),
    Info(String),
}

pub struct RegisterPlayer(pub Addr<GameSocket>);

pub struct MakeTurn {
    pub player: Addr<GameSocket>,
    pub turn_data: Position,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position {x, y}
    }
}

impl Message for MakeTurn {
    type Result = ();
}

impl Message for RegisterPlayer {
    type Result = bool;
}

impl Message for ClientMessage {
    type Result = ();
}