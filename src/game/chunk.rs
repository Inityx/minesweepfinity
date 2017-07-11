use std::fmt;
use rand::random;
use ::aux::index_iter;
use ::aux::coord::Coord;

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
    pub fn with_mines() -> Chunk {
        let mut chunk: Chunk = Default::default();
        
        let num_mines = random::<u8>()%(MAX_MINES - MIN_MINES) + MIN_MINES;
        for _ in 0..num_mines {
            // duplicate entries are not of consequence.
            chunk.enmine(
                Coord(
                    random::<usize>()%8,
                    random::<usize>()%8,
                )
            );
        }
        chunk.status = Status::Enmined;
        
        chunk
    }
    
    pub fn view(&self) -> Vec<SquareView> {
        let show_mines = self.status == Status::Won;
        
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
        index_iter::IndexIterUnsigned::new(Coord(8,8), Coord(0,0))
    }
    
    // Setters
    #[inline]
    pub fn enmine       (&mut self, coord: Coord<usize>) {
        self.mines[coord.0] |= 1u8<<(7-coord.1);
    }
    
    #[inline]
    pub fn click        (&mut self, coord: Coord<usize>) {
        self.visible[coord.0] |= 1u8<<(7-coord.1);
    }

    #[inline]
    pub fn enflag       (&mut self, coord: Coord<usize>) {
        self.flags[coord.0] |= 1u8<<(7-coord.1);
    }

    #[inline]
    pub fn deflag       (&mut self, coord: Coord<usize>) {
        self.flags[coord.0] &= (!1u8)<<(7-coord.1);
    }

    #[inline]
    pub fn set_neighbors(&mut self, coord: Coord<usize>, n: u32) {
        self.neighbors[coord.0] =
            (self.neighbors[coord.0] & !(15u32<<((7-coord.1)*4))) |
            n << ((7-coord.1)*4);
    }
    
    // Getters
    #[inline]
    pub fn is_mine      (&self, coord: Coord<usize>) -> bool {
        self.mines[coord.0] & 1u8<<(7-coord.1) == 1u8<<(7-coord.1)
    }

    #[inline]
    pub fn is_clicked   (&self, coord: Coord<usize>) -> bool {
        self.visible[coord.0] & 1u8<<(7-coord.1) == 1u8<<(7-coord.1)
    }
    
    #[inline]
    pub fn is_flag      (&self, coord: Coord<usize>) -> bool {
        self.flags[coord.0] & 1u8<<(7-coord.1) == 1u8<<(7-coord.1)
    }

    #[inline]
    pub fn get_neighbors(&self, coord: Coord<usize>) -> u32 {
        ((self.neighbors[coord.0] & 15u32<<((7-coord.1)*4))>>((7-coord.1)*4))
    }
}

impl fmt::Debug for Chunk {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let squareify = |x| String::from(if x {"[]"} else {"  "});
        
        let mut buffer = String::new();
        {
            let mut print_with = |name, func: &(Fn(Coord<usize>) -> String)| {
                buffer.push_str(name);
                buffer.push_str(":\n+----------------+\n");
                for i in 0..8 {
                    buffer.push('|');
                    for j in 0..8 {
                        buffer.push_str(&(func(Coord(i,j))));
                    }
                    buffer.push_str("|\n");
                }
                buffer.push_str("+----------------+\n");
            };
            
            print_with("Clicked",   &|coord| squareify(self.is_clicked(coord)));
            print_with("Flagged",   &|coord| squareify(self.is_flag(coord)));
            print_with("Neighbors", &|coord| {
                if self.is_mine(coord) { return String::from(" ¤"); }
                
                let neighbors = self.get_neighbors(coord);
                
                if neighbors > 0 {
                    format!(" {}", neighbors)
                } else {
                    String::from("  ")
                }
            });
        }
        
        fmt::Display::fmt(&buffer, f)
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
