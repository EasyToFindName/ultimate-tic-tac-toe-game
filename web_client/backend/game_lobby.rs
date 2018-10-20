use actix::prelude::*;

use game_socket::GameSocket;
use messages::{MakeTurn, RegisterPlayer, ClientMessage};

use tic_tac_toe::{game_board::*, game_elements::*};

static GLYPHS: [GameElem; 2] = [GameElem::X, GameElem::O];

enum GameState {
    Pending,
    Running,
    Terminating,
}

struct Player {
    addr: Addr<GameSocket>,
    glyph: GameElem,
}

pub struct GameLobby {
    game_board: GameBoard,
    players: Vec<Player>,
    current_player_ix: usize,
    capacity: usize,
    state: GameState,

}

impl GameLobby {
    pub fn new(capacity: usize) -> Self {
        GameLobby {
            game_board: GameBoard::new(10, 10, 5),
            current_player_ix: 0,
            players: Vec::with_capacity(capacity),
            state: GameState::Pending,
            capacity,
        }
    }

    fn add_player(&mut self, addr: Addr<GameSocket>) -> bool {
        let len = self.players.len();
        if len < self.capacity {
            self.players.push(Player{addr, glyph: GLYPHS[len]});

            if self.players.len() == self.capacity {
                self.state = GameState::Running;
            }

            true
        }
        else {
            false
        }
    }

    fn broadcast_message(&self, message: ClientMessage) {
        for player in &self.players {
            player.addr.do_send(message.clone());
        }
    }

}

impl Actor for GameLobby {
    type Context = Context<Self>;
}

impl Handler<MakeTurn> for GameLobby {
    type Result = ();

    fn handle(&mut self, msg: MakeTurn, ctx: &mut Context<Self>) {
        if let GameState::Pending = self.state {
            self.broadcast_message(ClientMessage::Info(String::from("Waiting for another player to start the game")));
            return;
        }

        let current_player = &self.players[self.current_player_ix];

        if msg.player != current_player.addr {
            msg.player.do_send(ClientMessage::Info(String::from("It's not your turn!")));
            return;
        }

        let result = self.game_board.set_if_free(msg.turn_data.x, msg.turn_data.y, current_player.glyph);

        if result == true {
            println!("Turn {:?} is possible", msg.turn_data);

            match current_player.glyph {
                GameElem::X => self.broadcast_message(ClientMessage::Cross(msg.turn_data)),
                GameElem::O => self.broadcast_message(ClientMessage::Circle(msg.turn_data)),
                _ => ()
            }

            self.current_player_ix = (self.current_player_ix + 1) % self.players.len();
        }
        else {
            println!("Turn {:?} is impossible", msg.turn_data);
        }
    }
}

impl Handler<RegisterPlayer> for GameLobby {
    type Result = bool;

    fn handle(&mut self, msg: RegisterPlayer, ctx: &mut Context<Self>) -> Self::Result {
        self.add_player(msg.0)
    }
}