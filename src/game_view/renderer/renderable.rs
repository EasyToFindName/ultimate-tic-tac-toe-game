use piston_window::{
    Graphics,
    draw_state::DrawState,
    math::Matrix2d
};

pub trait Renderable {
    fn draw<G: Graphics>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G);
}
