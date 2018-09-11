use piston_window::{draw_state::DrawState, math::Matrix2d, Graphics};

pub trait Renderable {
    fn draw<G: Graphics>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G);
}
