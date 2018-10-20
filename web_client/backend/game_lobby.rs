use actix::prelude::*;

use game_socket::GameSocket;
use messages::{MakeTurn, RegisterPlayer, ClientMessage};

use tic_tac_toe::{game_board::*, game_elements::*};

pub struct GameLobby {
    game_board: GameBoard,
    current_player: GameElem,
    players: Vec<Addr<GameSocket>>,
    capacity: usize,
}

impl GameLobby {
    pub fn new(capacity: usize) -> Self {
        GameLobby {
            game_board: GameBoard::new(10, 10, 5),
            current_player: GameElem::X,
            players: Vec::with_capacity(capacity),
            capacity
        }
    }

    fn add_player(&mut self, player: Addr<GameSocket>) -> bool {
        if self.players.len() < self.capacity {
            self.players.push(player);
            true
        }
        else {
            false
        }
    }
}

impl Actor for GameLobby {
    type Context = Context<Self>;
}

impl Handler<MakeTurn> for GameLobby {
    type Result = bool;

    fn handle(&mut self, msg: MakeTurn, ctx: &mut Context<Self>) -> Self::Result {
        let GameLobby {ref mut game_board,  ref mut current_player, ..} = *self;
        let result = game_board.set_if_free(msg.turn_data.x, msg.turn_data.y, *current_player);

        if result == true {
            println!("Turn {:?} is possible", msg.turn_data);
            *current_player = current_player.opposite();

            for player_addr in &self.players {
                player_addr.do_send(ClientMessage::Cross(msg.turn_data));
            }
        }
        else {
            println!("Turn {:?} is impossible", msg.turn_data);
        }

        result
    }
}

impl Handler<RegisterPlayer> for GameLobby {
    type Result = bool;

    fn handle(&mut self, msg: RegisterPlayer, ctx: &mut Context<Self>) -> Self::Result {
        self.add_player(msg.0)
    }
}