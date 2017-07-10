use std::fmt;
use rand::random;

const MIN_MINES: u8 = 8;
const MAX_MINES: u8 = 16;

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

pub enum SquareView {
    Unclicked {
        flag: bool,
        mine: bool
    },
    Clicked(u8),
}

#[derive(Default)]
pub struct Chunk {
    pub status: Status,
    mines:     [u8;8],
    visible:   [u8;8],
    flags:     [u8;8],
    neighbors: [u32;8],
}

#[allow(dead_code)]
impl Chunk {
    pub fn new() -> Chunk {
        let mut chunk: Chunk = Default::default();
        
        let num_mines = random::<u8>()%(MAX_MINES - MIN_MINES) + MIN_MINES;
        for _ in 0..num_mines {
            // duplicate entries are not of consequence.
            chunk.enmine(
                random::<usize>()%8,
                random::<usize>()%8
            );
        }
        chunk.status = Status::Enmined;
        
        chunk
    }
    
    pub fn blank() -> Chunk { Default::default() }
    
    pub fn iter_chunk_indeces() -> IterChunkIndeces { IterChunkIndeces(0) }
    pub fn iter_adjacent_indeces() -> IterAdjacentIndeces { IterAdjacentIndeces(0) }
    
    pub fn view(&self) -> Vec<SquareView> {
        let show_mines = self.status == Status::Won;
        
        Self::iter_chunk_indeces().map( |(i,j)|
            if self.is_clicked(i, j) {
                SquareView::Clicked(self.get_neighbors(i, j) as u8)
            } else {
                SquareView::Unclicked {
                    mine: show_mines && self.is_mine(i,j),
                    flag: self.is_flag(i,j),
                }
            }
        ).collect::<Vec<SquareView>>()
    }
    
    // Setters
    #[inline]
    pub fn enmine(&mut self, row: usize, col: usize) {
        self.mines[row] |= 1u8<<(7-col);
    }
    
    #[inline]
    pub fn click        (&mut self, row: usize, col: usize) {
        self.visible[row] |= 1u8<<(7-col);
    }

    #[inline]
    pub fn enflag       (&mut self, row: usize, col: usize) {
        self.flags[row] |= 1u8<<(7-col);
    }

    #[inline]
    pub fn deflag       (&mut self, row: usize, col: usize) {
        self.flags[row] &= (!1u8)<<(7-col);
    }

    #[inline]
    pub fn set_neighbors(&mut self, row: usize, col: usize, n: u32) {
        self.neighbors[row] = (self.neighbors[row] & !(15u32<<((7-col)*4))) | n << ((7-col)*4);
    }
    
    // Getters
    #[inline]
    pub fn is_mine      (&self, row: usize, col: usize) -> bool {
        self.mines[row] & 1u8<<(7-col) == 1u8<<(7-col)
    }

    #[inline]
    pub fn is_clicked   (&self, row: usize, col: usize) -> bool {
        self.visible[row] & 1u8<<(7-col) == 1u8<<(7-col)
    }
    
    #[inline]
    pub fn is_flag      (&self, row: usize, col: usize) -> bool {
        self.flags[row] & 1u8<<(7-col) == 1u8<<(7-col)
    }

    #[inline]
    pub fn get_neighbors(&self, row: usize, col: usize) -> u32 {
        ((self.neighbors[row] & 15u32<<((7-col)*4))>>((7-col)*4))
    }
}

impl fmt::Debug for Chunk {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let iter = |s: &mut String, name, func: &Fn(usize, usize) -> String| {
            s.push_str(name);
            s.push_str(":\n+----------------+\n");
            for i in 0..8 {
                s.push('|');
                for j in 0..8 {
                    s.push_str(&(func(i, j)));
                }
                s.push_str("|\n");
            }
            s.push_str("+----------------+\n");
        };
        
        let square = |x| -> String {
            String::from(if x { "[]" } else { "  " })
        };

        let mut b = String::new();
        iter(&mut b, "Clicked",   &|row, col| square(self.is_clicked(row, col)));
        iter(&mut b, "Flagged",   &|row, col| square(self.is_flag(row, col)));
        iter(&mut b, "Neighbors", &|row, col| {
            let x = if self.is_mine(row, col) {
                10
            } else {
                self.get_neighbors(row, col)
            };

            match x {
                1 ... 9 => String::from(format!(" {}", x)),
                10 => String::from(" ¤"),
                _ => String::from("  "),
            }
        });
        
        fmt::Display::fmt(&b, f)
    }
}


pub struct IterChunkIndeces(usize);

impl Iterator for IterChunkIndeces {
    type Item = (usize, usize);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= 64 { return None; }
        
        let ret = (self.0/8, self.0%8);
        self.0 += 1;
        Some(ret)
    }
}

pub struct IterAdjacentIndeces(isize);

impl Iterator for IterAdjacentIndeces {
    type Item = (isize, isize);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= 9 { return None; }
        
        let ret = ((self.0/3)-1, (self.0%3)-1);
        self.0 += 1;
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;
    
    #[test]
    fn test_neighbors_accessors() {
        let mut chunk = Chunk::new();

        chunk.set_neighbors(0,7,10);
        chunk.set_neighbors(0,6,5);

        assert_eq!(format!("{:b}", chunk.neighbors[0]), "1011010");
        assert_eq!(chunk.get_neighbors(0,7), 10);
        assert_eq!(chunk.get_neighbors(0,6), 5);
    }
    
    #[test]
    fn iter_indeces() {
        let chunk_iter = Chunk::iter_chunk_indeces();
        let mut full_vec = Vec::with_capacity(64);
        
        for i in 0..8 {
            for j in 0..8 {
                full_vec.push((i, j));
            }
        }
        
        let chunk_vec = chunk_iter.collect::<Vec<(usize, usize)>>();
        assert_eq!(chunk_vec, full_vec);
    }
    
    #[test]
    fn iter_adjacent() {
        let chunk_iter = Chunk::iter_adjacent_indeces();
        let mut full_vec = Vec::with_capacity(9);
        
        for i in -1..2 {
            for j in -1..2 {
                full_vec.push((i, j));
            }
        }
        
        let chunk_vec = chunk_iter.collect::<Vec<(isize, isize)>>();
        assert_eq!(chunk_vec, full_vec);
    }
}
