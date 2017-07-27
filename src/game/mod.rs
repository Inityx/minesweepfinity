mod chunk;

use std::collections::HashMap;

use self::chunk::Chunk;
use ::aux::index_iter;
use ::aux::coord::Coord;
use ::aux::{ModuloSignedExt, DivFloorSignedExt};


type Board = HashMap<Coord<isize>, Chunk>;

#[derive(Debug)]
pub enum SquareView {
    Clicked(u8),
    Unclicked,
    Flagged,
    Penalty,
    Points,
}

#[derive(Default)]
pub struct Game {
    chunks: Board,
    chunks_won: u64,
    chunks_lost: u64,
}

impl Game {
    pub fn get_chunks_won(&self) -> u64 { self.chunks_won }
    pub fn get_chunks_lost(&self) -> u64 { self.chunks_lost }
    
    pub fn get_chunk(&self, coord: Coord<isize>) -> Option<&Chunk> { self.chunks.get(&coord) }
    
    fn world_to_chunk_square(input_coord: Coord<isize>) -> (Coord<isize>, Coord<usize>) {
        let chunk_coord  = input_coord.div_floor(chunk::DIMENSION as isize);
        let square_coord = input_coord.modulo   (chunk::DIMENSION as isize);
        
        return (chunk_coord, Coord::from(square_coord));
    }
    
    fn allocate_with_surround(&mut self, chunk_coord: Coord<isize>, square_coord: Coord<usize>) {
        // Ensure first click is not a mine
        if !self.chunks.contains_key(&chunk_coord) {
            let mut insert = Chunk::with_mines();
            while insert.is_mine(square_coord) { insert = Chunk::with_mines(); }
        
            self.chunks.insert(chunk_coord, insert);
        }
        
        // Find unallocated adjacent chunks
        let unallocated_chunk_coords = index_iter::self_and_adjacent()
            .map(|offset| chunk_coord + offset)
            .filter(|chunk_coord| !self.chunks.contains_key(&chunk_coord))
            .collect::<Vec<Coord<isize>>>();
        
        for chunk_coord in unallocated_chunk_coords.into_iter() {
            self.chunks.insert(chunk_coord, Chunk::with_mines());
        }
    }
    
    pub fn touch(&mut self, world_coords: Vec<Coord<isize>>) -> Option<Vec<Coord<isize>>> {
        // 64 to avoid early small reallocations
        let mut to_click = Vec::with_capacity(64);
        
        for world_coord in world_coords {
            let (chunk_coord, square_coord) = Self::world_to_chunk_square(world_coord);
            // Allocate chunk & surround and calculate if not yet done
            self.allocate_with_surround(chunk_coord, square_coord);
            self.calc_neighbors(chunk_coord);
            
            {
                let touched_chunk = self.chunks.get_mut(&chunk_coord).unwrap();
                
                // Skip if square has already been clicked
                if touched_chunk.is_clicked(square_coord) { continue; }
                
                // Actually click
                touched_chunk.unflag(square_coord);
                touched_chunk.click(square_coord);
                
                if touched_chunk.is_mine(square_coord) {
                    touched_chunk.status = chunk::Status::Lost;
                    self.chunks_lost += 1;
                    return None;
                }
                
                if touched_chunk.is_won() {
                    touched_chunk.status = chunk::Status::Won;
                    self.chunks_won += 1;
                }
            }
            
            // If safe, add adjacent to fringe vector
            if self.chunks
                .get_mut(&chunk_coord)
                .unwrap()
                .get_neighbors(square_coord) == 0
            {
                //let extend_world_coords = index_iter::cardinal_adjacent()
                //    .map(|offset| *offset + world_coord);
                let extend_world_coords = index_iter::self_and_adjacent()
                    .filter(|offset| *offset != Coord(0,0))
                    .map(|offset| offset + world_coord);
                
                to_click.extend(extend_world_coords);
            }
        }
        
        return if to_click.len() > 0 {
            Some(to_click)
        } else {
            None
        };
    }
    
    pub fn toggle_flag(&mut self, world_coord: Coord<isize>) {
        let (chunk_coord, square_coord) = Self::world_to_chunk_square(world_coord);
        self.allocate_with_surround(chunk_coord, square_coord);
        let target_chunk = self.chunks.get_mut(&chunk_coord).unwrap();
        
        if !target_chunk.is_clicked(square_coord) {
            target_chunk.toggle_flag(square_coord);
        }
        
        if target_chunk.is_won() {
            target_chunk.status = chunk::Status::Won;
            self.chunks_won += 1;
        }
    }
    
    pub fn calc_neighbors(&mut self, coord: Coord<isize>) {
        assert!(
            index_iter::self_and_adjacent()
                .map(|offset| coord + offset )
                .all(|chunk_coord| self.chunks.contains_key(&chunk_coord))
        );

        let mut canvas: Chunk = Chunk::default();

        {
            let center = self.chunks.get(&coord).unwrap();
            
            if center.status != chunk::Status::Enmined { return; }
            
            let mut surround = Vec::<&Chunk>::with_capacity(9);
            surround.extend(
                index_iter::self_and_adjacent()
                    .map(|offset| coord + offset )
                    .map(|target| self.chunks.get(&target).unwrap())
            );
            let surround = surround; // make immutable
            
            for square_index in Chunk::iterate_index() {
                if !center.is_mine(square_index) {
                    let mut count = 0;
                    let square_index_i: Coord<isize> = Coord::from(square_index);
                    
                    for offset in index_iter::self_and_adjacent() {
                        let RENAME_ME = (
                            square_index_i + offset + chunk::DIMENSION as isize
                        ) / chunk::DIMENSION as isize;
                        
                        let local_square_index = (
                            square_index_i + offset +
                              (Coord(2,2)-RENAME_ME)*chunk::DIMENSION as isize
                        ) % chunk::DIMENSION as isize;
                        
                        let target_chunk_index = 3*RENAME_ME.0 + RENAME_ME.1;
                        let target_chunk = surround[target_chunk_index as usize];
                        if target_chunk.is_mine(Coord::from(local_square_index)) {
                            count += 1;
                        }
                    }
                    
                    canvas.set_neighbors(square_index, count);
                }
            }
        }

        let dest = self.chunks.get_mut(&coord).unwrap();
        dest.neighbors = canvas.neighbors;
        dest.status = chunk::Status::Neighbored;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn chunk_cascade() {
        let mut game: Game = Game::default();
        let touch_points = vec![
            Coord(0, 0),
            Coord(0, 2),
            Coord(1, 3),
            Coord(2, 1),
            Coord(1, 1)
        ];
        
        for coord in touch_points { game.touch(vec![coord]); }
        
        let active_count = game
            .chunks
            .values()
            .filter(|chunk| chunk.status == chunk::Status::Neighbored)
            .count();
        
        assert_eq!(game.chunks.len(), 25);
        assert_eq!(active_count, 5);
    }
}
