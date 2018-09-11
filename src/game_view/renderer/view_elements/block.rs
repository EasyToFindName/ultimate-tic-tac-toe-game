use piston_window::{
    rectangle::Rectangle, 
    draw_state::DrawState,
    Graphics,
    math::{
        Scalar, 
        Matrix2d
    }
};

use game_view::renderer::renderable::Renderable;


#[derive(Copy, Clone)]
pub struct Block {
    pub x: usize,
    pub y: usize,
    rect: Rectangle,
    units: Scalar,
}

impl Block {
    pub fn new(x: usize, y: usize, units: Scalar) -> Self {
        let rect = Rectangle::new([0.5, 0.5, 0.5, 1.0]);
        Block {x, y, rect, units}
    }
}

impl Renderable for Block {
    fn draw<G>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        let x1 = self.x as Scalar * self.units;
        let y1 = self.y as Scalar * self.units;

        self.rect.draw([x1, y1, self.units, self.units], draw_state, transform, g);
    }
}


