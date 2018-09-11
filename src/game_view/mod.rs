pub mod renderer;
pub use self::renderer::*;

use tic_tac_toe::{GameBoard, GameElem};
use piston_window::{Graphics, DrawState};
use math::{Matrix2d, Scalar};


pub struct GameView {
    grid: Grid,
}

impl GameView {
    pub fn new(board: &GameBoard, resolution_x: u32, resolution_y: u32) -> Self {
        let grid = Grid::new(board.rows(), board.cols(), resolution_x, resolution_y);
        GameView {grid}
    }

    //out row, column
    pub fn view_to_logic_coords(&self, x: Scalar, y: Scalar) -> (usize, usize) {
        (x as usize / self.grid.units() as usize, y as usize / self.grid.units() as usize)
    }

    pub fn draw<G>(&self, game_board: &GameBoard, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        self.grid.draw(draw_state, transform, g);

        let mut circle = Circle::new(0, 0, self.grid.units());
        let mut cross = Cross::new(0, 0, self.grid.units());

        for row in 0..game_board.rows() {
            for column in 0..game_board.cols() {
                
                match game_board.get(row, column) {
                    GameElem::X => {
                        cross.x = row;
                        cross.y = column;
                        cross.draw(draw_state, transform, g);
                    },
                    GameElem::O => {
                        circle.x = row;
                        circle.y = column;
                        circle.draw(draw_state, transform, g);
                    },
                    _ => ()
                }
            }
        }
    }
}
