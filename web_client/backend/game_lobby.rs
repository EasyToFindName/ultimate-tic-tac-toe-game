use actix::prelude::*;
use tic_tac_toe::{game_board::*, game_elements::*};


pub struct GameLobby {
    game_board: GameBoard,
    current_player: GameElem,
}

impl GameLobby {
    pub fn new() -> Self {
        GameLobby {
            game_board: GameBoard::new(10, 10, 5),
            current_player: GameElem::X,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TurnData {
    x: usize,
    y: usize,
}

impl TurnData {
    fn new(x: usize, y: usize) -> Self {
        TurnData {x, y}
    }
}

impl Message for TurnData {
    type Result = bool;
}


impl Actor for GameLobby {
    type Context = Context<Self>;
}


impl Handler<TurnData> for GameLobby {
    type Result = bool;

    fn handle(&mut self, data: TurnData, ctx: &mut Context<Self>) -> Self::Result {
        let GameLobby {ref mut game_board,  ref mut current_player} = *self;

        let result = game_board.set_if_free(data.x, data.y, *current_player);

        if result == true {
            println!("Turn {:?} is possible", data);
            *current_player = current_player.opposite();
        }
        else {
            println!("Turn {:?} is impossible", data);
        }

        result
    }
}