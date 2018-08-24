use piston_window::{
    line::Line, 
    draw_state::DrawState,
    Graphics,
    math::{
        Scalar, 
        Matrix2d
    }
};

use game_view::renderer::renderable::Renderable;

#[derive(Copy, Clone)]
pub struct Cross {
    x: usize,
    y: usize,
    line: Line,
    units: Scalar,
}

impl Cross {
    pub fn new(x: usize, y: usize, units: Scalar) -> Self {
        let line = Line::new([0.0, 0.0, 0.4, 0.9], 0.8);
        Cross{x, y, line, units}
    }
}

impl Renderable for Cross {
    fn draw<G>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        // up-left to bottom-right
        let x1 = self.x as Scalar * self.units;
        let y1 = self.y as Scalar * self.units;

        let x2 = x1 + self.units;
        let y2 = y1 + self.units;

        self.line.draw([x1, y1, x2, y2], draw_state, transform, g);

        //up-right to bottom-left
        let x1 = x1 + self.units;
        let x2 = x2 - self.units;

        self.line.draw([x1, y1, x2, y2], draw_state, transform, g);
    }
}
