use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameElem {
    X,
    O,
    Free,
    Obstacle,
}

impl GameElem {
    pub fn opposite(self) -> Self {
        match self {
            GameElem::X => GameElem::O,
            GameElem::O => GameElem::X,
            GameElem::Free => GameElem::Obstacle,
            GameElem::Obstacle => GameElem::Free,
        }
    }
}

impl fmt::Display for GameElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameElem::X => write!(f, "X"),
            GameElem::O => write!(f, "O"),
            GameElem::Free => write!(f, " "),
            GameElem::Obstacle => write!(f, "*"),
        }
    }
}
