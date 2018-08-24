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
pub struct Grid {
    cols: usize,
    rows: usize,
    units: Scalar,
    line: Line,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, resolution_x: u32, resolution_y: u32) -> Self {
        let row_unit = (resolution_y as Scalar) / (rows as Scalar);
        let col_unit = (resolution_x as Scalar) / (cols as Scalar);

        let mut units = row_unit;
        if col_unit < row_unit { units = col_unit; }

        let line = Line::new([0.0, 0.0, 0.5, 0.6], 0.5);

        Grid { rows, cols, units, line }
    }

    pub fn units(&self) -> Scalar { self.units }
}

impl Renderable for Grid {
    fn draw<G>(&self, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        let &Grid { cols, rows, units, line } = self;
        for x in 0..cols + 1 {
            let x1 = x as Scalar * units;
            let y1 = 0.0;
            let x2 = x1;
            let y2 = rows as Scalar * units;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
        for y in 0..rows + 1 {
            let x1 = 0.0;
            let y1 = y as Scalar * units;
            let x2 = cols as Scalar * units;
            let y2 = y1;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
    }
}

