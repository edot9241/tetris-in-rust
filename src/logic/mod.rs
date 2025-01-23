mod tetriminos;
use crate::logic::tetriminos::{Shape as TetriminoShape, Tetrimino};

pub(super) struct Game {
    matrix: Matrix,
    bag: Vec<TetriminoShape>,
}

impl Game {
    pub(super) fn new() -> Self {
        Game {
            matrix: Matrix::blank(),
        }
    }
}

struct Matrix([bool; Self::AREA]);

impl Matrix {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 22;
    const AREA: usize = Self::WIDTH * Self::HEIGHT;

    fn blank() -> Self {
        Self([false; Self::AREA])
    }
}
