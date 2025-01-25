use std::ops::Not;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl Not for Cell {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}
