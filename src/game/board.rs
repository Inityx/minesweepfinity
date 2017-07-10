use std::collections::HashMap;
use super::chunk;
use super::chunk::Chunk;

#[derive(Default)]
pub struct Board {
    allocated: u64,
    activated: u64,
    chunks: HashMap<(isize, isize), Chunk>,
}

impl Board {
    pub fn get_chunk(&self, row: isize, col: isize) -> Option<&Chunk> {
        self.chunks.get(&(row, col))
    }
    
    pub fn touch(&mut self, row: isize, col: isize) {
        // chunk cascade
        for (i, j) in Chunk::iter_adjacent_indeces() {
            if !self.chunks.contains_key(&(row+i, col+j)) {
                self.chunks.insert((row+i,col+j), Chunk::new());
                self.allocated += 1;
            }
        }
        self.calc_neighbors(row, col);
        self.activated += 1;
    }
    
    pub fn calc_neighbors(&mut self, row: isize, col: isize) {
        for (i, j) in Chunk::iter_adjacent_indeces() {
            assert!(self.chunks.contains_key(&(row+i, col+j)));
        }

        let mut canvas = Chunk::blank();

        {
            // borrow center and neighbors
            let center = self.chunks.get(
                &(row, col)
            ).unwrap();
            
            let mut surround = Vec::<&Chunk>::with_capacity(9);
            for (i, j) in Chunk::iter_adjacent_indeces() {
                surround.push(
                    self.chunks.get(
                        &(row+i, col+j)
                    ).unwrap()
                );
            }
            let surround = surround; // make immutable
            
            for (i, j) in Chunk::iter_chunk_indeces() {
                if !center.is_mine(i, j) {
                    let mut count = 0;
                    let ii = i as isize;
                    let ji = j as isize;
                    
                    for (k, l) in Chunk::iter_adjacent_indeces() {
                        let r = (ii+k+8)/8; // surround row
                        let c = (ji+l+8)/8; // surround column
                        let local_row = (ii+k+8*(2-r))%8;
                        let local_col = (ji+l+8*(2-c))%8;
                        
                        count += surround[(3*r+c) as usize].is_mine(
                            local_row as usize,
                            local_col as usize,
                        ) as u32;
                    }
                    
                    canvas.set_neighbors(i, j, count);
                }
            }
        }

        let dest = self.chunks.get_mut(&(row, col)).unwrap();
        *dest = canvas;
        dest.status = chunk::Status::Neighbored;
    }
    
    pub fn chunk_view(&self, row: isize, col: isize) -> Vec<chunk::SquareView> {
        self.chunks.get(&(row, col)).unwrap().view()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn chunk_cascade() {
        let mut board: Board = Default::default();
        for &(row, col) in [(0,0), (0,2), (1,3), (2,1), (1,1)].into_iter() {
            board.touch(row, col);
        }
        
        assert_eq!(board.allocated, 25);
        assert_eq!(board.activated, 5);
    }
}
