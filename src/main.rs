extern crate tic_tac_toe;
extern crate piston_window;

mod game_view;
mod input;
mod game;


use piston_window::*;
use tic_tac_toe::*;
use input::*;
use game::*;

const RESOLUTION_X: u32 = 640;
const RESOLUTION_Y: u32 = 640;

const BOARD_SIZE_X: usize = 32;
const BOARD_SIZE_Y: usize = 32;
const WIN_SEQ_LEN: usize = 5;

fn main() {
    let game_board = GameBoard::new(BOARD_SIZE_X, BOARD_SIZE_Y, WIN_SEQ_LEN);
    let mut game = Game::new(game_board, RESOLUTION_X, RESOLUTION_Y);

    let mut window: PistonWindow = PistonWindow::new(
        OpenGL::V3_3,
        0,
        WindowSettings::new("Hello World!", [RESOLUTION_X, RESOLUTION_Y])
            .opengl(OpenGL::V3_3)
            .samples(4)
            .srgb(false)
            .build()
            .unwrap(),
    );

    while let Some(e) = window.next() {
        game.process_event(&e);

        window.draw_2d(&e, |c, g| {
            game.draw(&c.draw_state, c.transform, g);
        });
    }
}