use rand::random;
use crate::{
    aux::{index_iter, coord::Coord},
    game::SquareView,
};

const MIN_MINES: u8 = 8;
const MAX_MINES: u8 = 16;

pub const DIMENSION: usize = 8;

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
    mines:         [u8; DIMENSION],
    clicked:       [u8; DIMENSION],
    flags:         [u8; DIMENSION],
    pub neighbors: [u32;DIMENSION],
}

#[allow(dead_code)]
impl Chunk {
    pub fn with_mines() -> Chunk {
        let mut chunk = Chunk::default();
        
        let num_mines = random::<u8>() % (MAX_MINES - MIN_MINES) + MIN_MINES;
        for _ in 0..num_mines {
            // Enmining is idempotent, duplicate entries don't matter.
            chunk.enmine(
                Coord(
                    random::<usize>() % DIMENSION,
                    random::<usize>() % DIMENSION,
                )
            );
        }
        chunk.status = Status::Enmined;
        
        return chunk;
    }
    
    pub fn view(&self) -> Vec<SquareView> {
        Self::iterate_index().map( |coord|
            if self.is_clicked(coord) {
                if self.is_mine(coord) {
                    SquareView::Penalty
                } else {
                    SquareView::Clicked(self.get_neighbors(coord) as u8)
                }
            } else {
                if self.status == Status::Won {
                    SquareView::Points
                } else if self.status == Status::Lost && self.is_mine(coord) {
                    SquareView::Penalty
                } else if self.is_flag(coord) {
                    SquareView::Flagged
                } else {
                    SquareView::Unclicked
                }
            }
        ).collect::<Vec<SquareView>>()
    }
    
    pub fn iterate_index() -> index_iter::IndexIterUnsigned {
        index_iter::IndexIterUnsigned::new(
            Coord(DIMENSION, DIMENSION),
            Coord(0, 0)
        )
    }
    
    pub fn is_won(&self) -> bool {
        // all mines are flagged
        self.mines == self.flags &&
        // clicks are inverse of mines
        self.mines
            .iter()
            .zip(self.clicked.iter())
            .all(|(mine, clicked)| (!*mine) == *clicked )
    }
    
    // Setters
    // TODO: Macros?
    #[inline]
    pub fn enmine       (&mut self, coord: Coord<usize>) {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.mines[coord.0] |= 0x01u8<<(7-coord.1);
    }
    
    #[inline]
    pub fn click        (&mut self, coord: Coord<usize>) {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.clicked[coord.0] |= 0x01u8<<(7-coord.1);
    }

    #[inline]
    pub fn toggle_flag  (&mut self, coord: Coord<usize>) {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.flags[coord.0] ^= 0x01u8<<(7-coord.1);
    }
    
    #[inline]
    pub fn unflag  (&mut self, coord: Coord<usize>) {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.flags[coord.0] &= !(0x01u8<<(7-coord.1));
    }

    #[inline]
    pub fn set_neighbors(&mut self, coord: Coord<usize>, n: u32) {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.neighbors[coord.0] =
            (self.neighbors[coord.0] & !(0x0Fu32<<((7-coord.1)*4))) |
            n << ((7-coord.1)*4);
    }
    
    // Getters
    #[inline]
    pub fn is_mine      (&self, coord: Coord<usize>) -> bool {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.mines[coord.0] & 0x01u8<<(7-coord.1) == 0x01u8<<(7-coord.1)
    }

    #[inline]
    pub fn is_clicked   (&self, coord: Coord<usize>) -> bool {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.clicked[coord.0] & 0x01u8<<(7-coord.1) == 0x01u8<<(7-coord.1)
    }
    
    #[inline]
    pub fn is_flag      (&self, coord: Coord<usize>) -> bool {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.flags[coord.0] & 0x01u8<<(7-coord.1) == 0x01u8<<(7-coord.1)
    }

    #[inline]
    pub fn get_neighbors(&self, coord: Coord<usize>) -> u32 {
        debug_assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        ((self.neighbors[coord.0] & 0x0Fu32<<((7-coord.1)*4))>>((7-coord.1)*4))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_neighbors_accessors() {
        let mut chunk = Chunk::with_mines();

        chunk.set_neighbors(Coord(0,7),10);
        chunk.set_neighbors(Coord(0,6),5);

        assert_eq!(format!("{:b}", chunk.neighbors[0]), "1011010");
        assert_eq!(chunk.get_neighbors(Coord(0,7)), 10);
        assert_eq!(chunk.get_neighbors(Coord(0,6)), 5);
    }
}
