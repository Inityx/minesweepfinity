pub mod field;

use rand::random;
use crate::{
    aux::{index_iter, coord::Coord},
    game::SquareView,
};
use self::field::{BitField, NybbleField};

use std::ops::Not;

pub use self::field::DIMENSION;

const MIN_MINES: u8 = 8;
const MAX_MINES: u8 = 16;


#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    Blank,
    Enmined,
    Neighbored,
    Won,
    Lost,
}

impl Default for Status {
    fn default() -> Self { Status::Blank }
}

#[derive(Default)]
pub struct Chunk {
    pub status: Status,
    pub mines:     BitField,
    pub clicked:   BitField,
    pub flags:     BitField,
    pub neighbors: NybbleField,
}

impl Chunk {
    pub fn with_mines() -> Chunk {
        let num_mines = random::<u8>() % (MAX_MINES - MIN_MINES) + MIN_MINES;
        let mut mines = BitField::default();

        for _ in 0..num_mines { mines.set(random_square()) }
        
        Chunk {
            status: Status::Enmined,
            mines,
            ..Chunk::default()
        }
    }
    
    pub fn view(&self, square: Coord<usize>) -> SquareView {
        if self.clicked.get(square) {
            if self.mines.get(square) {
                SquareView::Penalty
            } else {
                SquareView::Clicked(self.neighbors.get(square))
            }
        } else {
            if self.status == Status::Won {
                SquareView::Points
            } else if self.status == Status::Lost && self.mines.get(square) {
                SquareView::Penalty
            } else if self.flags.get(square) {
                SquareView::Flagged
            } else {
                SquareView::Unclicked
            }
        }
    }
    
    pub fn is_won(&self) -> bool {
        self.mines == self.flags && self.mines.not() == self.clicked
    }
}

pub fn all_squares() -> index_iter::IndexIterUnsigned {
    index_iter::IndexIterUnsigned::new(
        Coord::squared(field::DIMENSION),
        Coord::default(),
    )
}

pub fn random_square() -> Coord<usize> {
    Coord(
        random::<usize>() % DIMENSION,
        random::<usize>() % DIMENSION,
    )
}
    
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_neighbors_accessors() {
        let mut chunk = Chunk::with_mines();

        chunk.neighbors.set(Coord(0,7), 10);
        chunk.neighbors.set(Coord(0,6), 5);

        assert_eq!(chunk.neighbors.get(Coord(0,7)), 10);
        assert_eq!(chunk.neighbors.get(Coord(0,6)), 5);
    }
}
