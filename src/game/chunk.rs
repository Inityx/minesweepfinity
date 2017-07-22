use std::fmt;
use rand::random;
use ::aux::index_iter;
use ::aux::coord::Coord;
use super::SquareView;

const MIN_MINES: u8 = 8;
const MAX_MINES: u8 = 16;

pub const DIMENSION: usize = 8;

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    Blank,
    Enmined,
    Neighbored,
    Won,
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
        
        let num_mines = random::<u8>()%(MAX_MINES - MIN_MINES) + MIN_MINES;
        for _ in 0..num_mines {
            // duplicate entries are not of consequence.
            chunk.enmine(
                Coord(
                    random::<usize>()%DIMENSION,
                    random::<usize>()%DIMENSION,
                )
            );
        }
        chunk.status = Status::Enmined;
        
        chunk
    }
    
    pub fn view(&self) -> Vec<SquareView> {
        // let show_mines = self.status == Status::Won;
        let show_mines = true;
        
        Self::iterate_index().map( |coord|
            if self.is_clicked(coord) {
                SquareView::Clicked(self.get_neighbors(coord) as u8)
            } else {
                SquareView::Unclicked {
                    mine: show_mines && self.is_mine(coord),
                    flag: self.is_flag(coord),
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
        // all non-mines are clicked
        self.mines
            .iter()
            .zip(self.clicked.iter())
            .all(|(mine, clicked)| (!*mine) == *clicked )
    }
    
    // Setters
    // TODO: Macros?
    #[inline]
    pub fn enmine       (&mut self, coord: Coord<usize>) {
        assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.mines[coord.0] |= 0x01u8<<(7-coord.1);
    }
    
    #[inline]
    pub fn click        (&mut self, coord: Coord<usize>) {
        assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.clicked[coord.0] |= 0x01u8<<(7-coord.1);
    }

    #[inline]
    pub fn toggle_flag  (&mut self, coord: Coord<usize>) {
        assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.flags[coord.0] ^= 0x01u8<<(7-coord.1);
    }

    #[inline]
    pub fn set_neighbors(&mut self, coord: Coord<usize>, n: u32) {
        assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.neighbors[coord.0] =
            (self.neighbors[coord.0] & !(0x0Fu32<<((7-coord.1)*4))) |
            n << ((7-coord.1)*4);
    }
    
    // Getters
    #[inline]
    pub fn is_mine      (&self, coord: Coord<usize>) -> bool {
        assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.mines[coord.0] & 0x01u8<<(7-coord.1) == 0x01u8<<(7-coord.1)
    }

    #[inline]
    pub fn is_clicked   (&self, coord: Coord<usize>) -> bool {
        assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.clicked[coord.0] & 0x01u8<<(7-coord.1) == 0x01u8<<(7-coord.1)
    }
    
    #[inline]
    pub fn is_flag      (&self, coord: Coord<usize>) -> bool {
        assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
        self.flags[coord.0] & 0x01u8<<(7-coord.1) == 0x01u8<<(7-coord.1)
    }

    #[inline]
    pub fn get_neighbors(&self, coord: Coord<usize>) -> u32 {
        assert!(coord.0 < DIMENSION && coord.1 < DIMENSION);
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
