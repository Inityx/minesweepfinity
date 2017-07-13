use std::collections::HashMap;
use super::chunk;
use super::chunk::Chunk;
use ::aux::index_iter;
use ::aux::coord::Coord;

#[derive(Default)]
pub struct Board {
    allocated: u64,
    activated: u64,
    chunks: HashMap<Coord<isize>, Chunk>,
}

impl Board {
    pub fn get_chunk(&self, coord: &Coord<isize>) -> Option<&Chunk> {
        self.chunks.get(coord)
    }
    
    pub fn touch(&mut self, coord: &Coord<isize>) {
        // chunk cascade
        for offset in index_iter::adjacent() {
            if !self.chunks.contains_key(&(coord + offset)) {
                self.chunks.insert(coord + offset, Chunk::with_mines());
                self.allocated += 1;
            }
        }
        self.calc_neighbors(coord);
        self.activated += 1;
    }
    
    pub fn calc_neighbors(&mut self, coord: &Coord<isize>) {
        assert!(
            index_iter::adjacent()
                .map(|offset| coord + offset )
                .all(|target| self.chunks.contains_key(&target))
        );

        let mut canvas: Chunk = Default::default();

        {
            let center = self.chunks.get(&coord).unwrap();
            
            let mut surround = Vec::<&Chunk>::with_capacity(9);
            surround.extend(
                index_iter::adjacent()
                    .map(|offset| coord + offset )
                    .map(|target| self.chunks.get(&target).unwrap())
            );
            let surround = surround; // make immutable
            
            for square_index in Chunk::iterate_index() {
                if !center.is_mine(square_index) {
                    let mut count = 0;
                    let square_index_i: Coord<isize> = From::from(square_index);
                    
                    for offset in index_iter::adjacent() {
                        let RENAME_ME = (square_index_i + offset + Coord(8,8))/Coord(8,8);
                        
                        let local_square_index = (
                            square_index_i + offset +
                            Coord(8isize,8)*(Coord(2isize,2)-RENAME_ME)
                        )%Coord(8,8);
                        
                        let target_chunk_index = 3*RENAME_ME.0 + RENAME_ME.1;
                        let target_chunk = surround[target_chunk_index as usize];
                        let is_mine = target_chunk.is_mine(From::from(local_square_index));
                        count += is_mine as u32;
                    }
                    
                    canvas.set_neighbors(square_index, count);
                }
            }
        }

        let dest = self.chunks.get_mut(coord).unwrap();
        dest.neighbors = canvas.neighbors;
        dest.status = chunk::Status::Neighbored;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn chunk_cascade() {
        let mut board: Board = Default::default();
        let touch_points = vec![
            Coord(0,0),
            Coord(0,2),
            Coord(1,3),
            Coord(2,1),
            Coord(1,1)
        ];
        
        for ref coord in touch_points { board.touch(coord); }
        
        assert_eq!(board.allocated, 25);
        assert_eq!(board.activated, 5);
    }
}
