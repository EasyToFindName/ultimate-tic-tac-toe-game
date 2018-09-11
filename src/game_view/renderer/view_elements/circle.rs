use piston_window::{
    ellipse::Ellipse, 
    draw_state::DrawState,
    Graphics,
    math::{
        Scalar, 
        Matrix2d
    }
};

use game_view::renderer::renderable::Renderable;

#[derive(Copy, Clone)]
pub struct Circle {
    pub x: usize,
    pub y: usize,
    ellipse: Ellipse,
    units: Scalar,
}

impl Circle {
    pub fn new(x: usize, y: usize, units: Scalar) -> Self {
        let ellipse = Ellipse::new_border([0.35, 0.35, 0.5, 1.0], 0.90);
        Circle{x, y, ellipse, units}
    }
}

impl Renderable for Circle {
    fn draw<G>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {

        let offset = self.units / 8.0;

        let x1 = self.x as Scalar * self.units + (offset / 2.0);
        let y1 = self.y as Scalar * self.units + (offset / 2.0);

        let x2 = self.units - offset;
        let y2 = self.units - offset;

        self.ellipse.draw([x1, y1, x2, y2], draw_state, transform, g);
    }
}
