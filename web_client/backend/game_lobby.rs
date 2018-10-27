use actix::prelude::*;
use game_socket::GameSocket;

use messages::{
    Position, MakeTurn, RegisterPlayer, ClientMessage,
    PlayerDisconnected, LobbyClosed
};

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
            players: Vec::with_capacity(capacity),
            current_player_ix: 0,
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

    fn remove_player(&mut self, addr: &Addr<GameSocket>) {
        self.players.retain(|player| player.addr != *addr);
    }

    fn broadcast_message<M>(&self, message: M)
    where
        M: Message + Clone + Send + 'static,
        <M as actix::Message>::Result: Send,
        GameSocket: actix::Handler<M>,
    {
        for player in &self.players {
            player.addr.do_send(message.to_owned());
        }
    }

    fn reset(&mut self) {
        *self = GameLobby::new(self.capacity);
    }

}

impl Actor for GameLobby {
    type Context = Context<Self>;
}

impl Handler<MakeTurn> for GameLobby {
    type Result = ();

    fn handle(&mut self, msg: MakeTurn, _ctx: &mut Context<Self>) {
        // check if turn is legal
        match self.state {
            GameState::Pending => {
                self.broadcast_message(ClientMessage::Info(String::from("Waiting for antoher player to start the game")));
                return;
            },
            GameState::Running => (),
            GameState::Terminating => {
                self.broadcast_message(ClientMessage::Info(String::from("The game is already over")));
                return;
            },
        }

        let current_player = &self.players[self.current_player_ix];

        if msg.player_addr != current_player.addr {
            msg.player_addr.do_send(ClientMessage::Info(String::from("It's not your turn!")));
            return;
        }

        if !self.game_board.set_if_free(msg.turn_data.y, msg.turn_data.x, current_player.glyph) {
            return;
        }

        // if turn was legal => update views
        match current_player.glyph {
            GameElem::X => self.broadcast_message(ClientMessage::Cross(msg.turn_data)),
            GameElem::O => self.broadcast_message(ClientMessage::Circle(msg.turn_data)),
            _ => ()
        }

        // check for a winner
        let winner = self.game_board.get_winner(msg.turn_data.y, msg.turn_data.x);

        // if winner was found, finish the game, otherwise switch player and continue
        match winner {
            Some(winner_data) => {
                let info = format!("The winner is {}", winner_data.elem);
                let line_p1 = Position::from(winner_data.win_line.0);
                let line_p2 = Position::from(winner_data.win_line.1);

                self.broadcast_message(ClientMessage::Line(line_p1, line_p2));
                self.broadcast_message(ClientMessage::Info(info));
                self.broadcast_message(LobbyClosed);
            }
            None => {
                self.current_player_ix = (self.current_player_ix + 1) % self.players.len();
            }
        }
    }
}


impl Handler<RegisterPlayer> for GameLobby {
    type Result = bool;

    fn handle(&mut self, msg: RegisterPlayer, _ctx: &mut Context<Self>) -> Self::Result {
        if let GameState::Pending = self.state {
            self.add_player(msg.0)
        }
        else {
            false
        }
    }
}

impl Handler<PlayerDisconnected> for GameLobby {
    type Result = ();

    fn handle(&mut self, msg: PlayerDisconnected, _ctx: &mut Context<Self>) {
        self.remove_player(&msg.0);

        if let GameState::Running = self.state {
            self.broadcast_message(ClientMessage::Info(String::from("Your opponent disconnected")));
            self.broadcast_message(LobbyClosed);
            self.state = GameState::Terminating;
        }

        if self.players.is_empty() {
//            ctx.stop();
            self.reset();
        }
    }
}
