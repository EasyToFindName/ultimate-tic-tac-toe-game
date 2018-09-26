use basics::*;
use game_elements::*;

use std::fmt;

// Directions used in get_winner algorithm
const DIRECTIONS: [Point<i32>; 4] = [
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
];

pub struct GameBoard {
    board: Arr2D<GameElem>,
    free_cells: usize,
    win_sequence_len: usize,
}

// Iterates on a sequence of identical GameElems inside a GameBoard in selected direciton
struct GameBoardSequenceIter<'a> {
    board: &'a GameBoard,
    direction: Point<i32>,
    pos: Point<usize>,
    value: GameElem,
}

impl GameBoard {
    pub fn new(rows: usize, columns: usize, win_sequence_len: usize) -> Self {
        GameBoard {
            board: Arr2D::new(rows, columns, GameElem::Free),
            free_cells: rows * columns,
            win_sequence_len,
        }
    }

    pub fn get(&self, row: usize, column: usize) -> GameElem {
        *self.board.get(row, column).unwrap()
    }

    pub fn get_optional(&self, row: usize, column: usize) -> Option<GameElem> {
        match self.board.get(row, column) {
            Ok(&elem) => Some(elem),
            Err(_) => None,
        }
    }

    pub fn set(&mut self, row: usize, column: usize, value: GameElem) {
        let old_value = self.get(row, column);
        if old_value == value {
            return;
        }

        match value {
            GameElem::Free => self.free_cells += 1,
            _ => self.free_cells -= 1,
        }

        self.board.set(row, column, value).unwrap();
    }

    pub fn set_if_free(&mut self, row: usize, column: usize, value: GameElem) -> bool {
        let old_value = self.get_optional(row, column);
        if let None = old_value {
            return false;
        }

        let old_value = old_value.unwrap();

        if let GameElem::Free = old_value {
            self.set(row, column, value);
            true
        } else {
            false
        }
    }

    pub fn is_draw(&self) -> bool {
        self.free_cells == 0
    }

    pub fn get_winner(&self, row: usize, column: usize) -> Option<GameElem> {
        let winner = self.get(row, column);

        if winner == GameElem::Free || winner == GameElem::Obstacle {
            return None;
        }

        for dir in DIRECTIONS.iter() {
            let sequence_iter = self.get_sequence_iter(row, column, *dir);
            let edge = sequence_iter.last().unwrap();

            let sequence_iter = self.get_sequence_iter(edge.y, edge.x, dir.inversed());

            let seq_len = sequence_iter.count();

            if seq_len >= self.win_sequence_len {
                return Some(winner);
            }
        }

        None
    }

    pub fn rows(&self) -> usize {
        self.board.rows()
    }
    pub fn cols(&self) -> usize {
        self.board.columns()
    }

    fn get_sequence_iter(
        &self,
        row: usize,
        column: usize,
        direction: Point<i32>,
    ) -> GameBoardSequenceIter {
        GameBoardSequenceIter::new(self, Point::new(column, row), direction)
    }
}

impl<'a> GameBoardSequenceIter<'a> {
    fn new(board: &'a GameBoard, start_pos: Point<usize>, direction: Point<i32>) -> Self {
        GameBoardSequenceIter {
            board,
            direction,
            pos: start_pos,
            value: board.get(start_pos.y, start_pos.x),
        }
    }
}

impl<'a> Iterator for GameBoardSequenceIter<'a> {
    type Item = Point<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let board_val = match self.board.get_optional(self.pos.y, self.pos.x) {
            Some(val) => val,
            None => return None,
        };

        if board_val == self.value {
            let old_pos = self.pos;

            self.pos.x = (self.pos.x as i32 + self.direction.x) as usize;
            self.pos.y = (self.pos.y as i32 + self.direction.y) as usize;

            Some(old_pos)
        } else {
            None
        }
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.board.rows() {
            for j in 0..self.board.columns() {
                write!(f, "{} ", self.get(i, j))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_iterator() {
        let g = GameBoard::new(3, 3, 3);

        let it = GameBoardSequenceIter::new(&g, Point::new(1, 1), DIRECTIONS[0]);

        assert_eq!(it.value, GameElem::Free);

        let last_element = it.last().unwrap();
        assert_eq!(last_element, Point::new(0, 1));
    }

    #[test]
    fn get_winner() {
        let mut g = GameBoard::new(3, 3, 3);

        let _ = g.set(0, 0, GameElem::X);
        let _ = g.set(1, 1, GameElem::X);
        let _ = g.set(2, 2, GameElem::X);

        assert_eq!(g.get_winner(0, 0), Some(GameElem::X));
        assert_eq!(g.get_winner(0, 2), None);
    }
}
