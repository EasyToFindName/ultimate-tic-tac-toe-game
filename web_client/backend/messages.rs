use actix::prelude::*;
use game_socket::GameSocket;
use tic_tac_toe::basics::point::Point;
use std::convert::From;

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

impl From<Point<usize>> for Position {
    fn from(point: Point<usize>) -> Self {
        Position {x: point.x, y: point.y}
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