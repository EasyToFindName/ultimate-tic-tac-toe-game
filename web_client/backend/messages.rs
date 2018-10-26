use actix::prelude::*;
use game_socket::GameSocket;

// helper struct
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position {x, y}
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientMessage {
    Cross(Position),
    Circle(Position),
    Line(Position, Position),
    Info(String),
}

pub struct RegisterPlayer(pub Addr<GameSocket>);
pub struct PlayerDisconnected(pub Addr<GameSocket>);

#[derive(Clone)]
pub struct LobbyClosed;

pub struct MakeTurn {
    pub player_addr: Addr<GameSocket>,
    pub turn_data: Position,
}

impl Message for ClientMessage {
    type Result = ();
}

impl Message for RegisterPlayer {
    type Result = bool;
}

impl Message for PlayerDisconnected {
    type Result = ();
}

impl Message for MakeTurn {
    type Result = ();
}

impl Message for LobbyClosed {
    type Result = ();
}