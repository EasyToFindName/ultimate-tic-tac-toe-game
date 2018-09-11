use game_view::*;
use input::*;
use piston_window::*;
use tic_tac_toe::*;

use draw_state::DrawState;
use math::Matrix2d;
use Graphics;

pub struct Game {
    game_board: GameBoard,
    game_view: GameView,
    input: InputMapper,
}

impl Game {
    pub fn new(game_board: GameBoard, resolution_x: u32, resolution_y: u32) -> Self {
        let game_view = GameView::new(&game_board, resolution_x, resolution_y);
        let input = InputMapper::new();

        Game {
            game_board,
            game_view,
            input,
        }
    }

    pub fn draw<G: Graphics>(&mut self, draw_state: &DrawState, transform: Matrix2d, g: &mut G) {
        clear([1.0; 4], g);

        self.game_view
            .draw(&self.game_board, draw_state, transform, g);
    }

    pub fn process_event(&mut self, e: &Event) {
        let mapped_event = self.input.process_event(&e);

        if let InputMapperEvent::MousePressed(button, x, y) = mapped_event {
            let (row, column) = self.game_view.view_to_logic_coords(x, y);
            self.game_board.set_if_free(row, column, GameElem::X);
        }
    }
}
