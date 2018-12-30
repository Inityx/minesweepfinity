mod chunk;

use self::chunk::Chunk;
use crate::aux::{
    index_iter::IndexIterSigned,
    coord::Coord,
    ModuloSignedExt,
    DivFloorSignedExt,
    OptionalizeExt,
};

type Board = hashbrown::HashMap<Coord<isize>, Chunk>;

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

fn world_to_chunk_square(input_coord: Coord<isize>) -> (Coord<isize>, Coord<usize>) {
    let chunk_coord  = input_coord.map(|x| x.div_floor(chunk::DIMENSION as isize));
    let square_coord = input_coord.map(|x| x.modulo   (chunk::DIMENSION as isize)).into();
    
    (chunk_coord, square_coord)
}
    
impl Game {
    pub fn new() -> Self { Game::default() }
    pub fn chunks_won(&self) -> u64 { self.chunks_won }
    pub fn chunks_lost(&self) -> u64 { self.chunks_lost }
    pub fn chunks(&self) -> &Board { &self.chunks }
    
    fn allocate_with_surround(&mut self, chunk: Coord<isize>, square: Coord<usize>) {
        use hashbrown::hash_map::Entry::Vacant;

        if let Vacant(entry) = self.chunks.entry(chunk) {
            entry.insert(loop {
                // Ensure first click is not a mine
                let insert = Chunk::with_mines();
                if insert.is_mine(square) { continue; }
                break insert;
            });
        }
        
        IndexIterSigned::self_and_adjacent(chunk)
            .filter(|&coord| coord != chunk)
            .for_each(|coord|
                if let Vacant(entry) = self.chunks.entry(coord) {
                    entry.insert(Chunk::with_mines());
                }
            );
    }
    
    pub fn touch(&mut self, world_coords: &[Coord<isize>]) -> Option<Vec<Coord<isize>>> {
        let mut to_click = Vec::with_capacity(64);
        
        for &world_coord in world_coords {
            let (chunk, square) = world_to_chunk_square(world_coord);
            // Allocate chunk & surround and calculate if not yet done
            self.allocate_with_surround(chunk, square);
            self.calc_neighbors(chunk);
            
            {
                let touched_chunk = self.chunks.get_mut(&chunk).unwrap();
                
                // Skip if square has already been clicked
                if touched_chunk.is_clicked(square) { continue; }
                
                // Actually click
                touched_chunk.unflag(square);
                touched_chunk.click(square);
                
                if touched_chunk.is_mine(square) {
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
            if self
                .chunks
                .get_mut(&chunk)
                .unwrap()
                .get_neighbors(square) == 0
            {
                let fringe = IndexIterSigned::self_and_adjacent(Coord::<isize>(0,0))
                    .filter(|&offset| offset != Coord(0,0))
                    .map(|offset| offset + world_coord);
                
                to_click.extend(fringe);
            }
        }
        
        to_click.optionalize()
    }
    
    pub fn toggle_flag(&mut self, world_coord: Coord<isize>) {
        let (chunk_coord, square_coord) = world_to_chunk_square(world_coord);
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
            IndexIterSigned::self_and_adjacent(coord)
                .all(|chunk_coord| self.chunks.contains_key(&chunk_coord))
        );

        let mut canvas: Chunk = Chunk::default();

        {
            let center = self.chunks.get(&coord).unwrap();
            
            if center.status != chunk::Status::Enmined { return; }
            
            let mut surround = Vec::<&Chunk>::with_capacity(9);
            surround.extend(
                IndexIterSigned::self_and_adjacent(coord)
                    .map(|target| self.chunks.get(&target).unwrap())
            );
            let surround = surround; // make immutable
            
            for square_index in Chunk::iterate_index() {
                if !center.is_mine(square_index) {
                    let mut count = 0;
                    let square_index_i: Coord<isize> = Coord::from(square_index);
                    
                    for offset in IndexIterSigned::self_and_adjacent(Coord::<isize>(0, 0)) {
                        let RENAME_ME = (
                            square_index_i + offset.map(|x| x + chunk::DIMENSION as isize)
                        ).map(|x| x / chunk::DIMENSION as isize);
                        
                        let local_square_index = (
                            square_index_i + offset +
                              (Coord(2,2)-RENAME_ME).map(|x| x *chunk::DIMENSION as isize)
                        ).map(|x| x % chunk::DIMENSION as isize);
                        
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
        let touch_points: [Coord<isize>;5] = [
            Coord( 0,  0),
            Coord( 0, 16),
            Coord( 8, 24),
            Coord(16,  8),
            Coord( 8,  8),
        ];
        
        for &coord in &touch_points {
            game.touch(&[coord]);
        }
        
        let active_count = game
            .chunks
            .values()
            .filter(|chunk| chunk.status != chunk::Status::Enmined)
            .count();
        
        assert_eq!(game.chunks.len(), 25);
        assert_eq!(active_count, 5);
    }
}
