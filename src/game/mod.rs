pub mod chunk;

use self::chunk::{Chunk, field::NybbleField};
use crate::aux::{
    index_iter::IndexIterSigned,
    coord::Coord,
    ModuloSignedExt,
    DivFloorSignedExt,
    OptionalizeExt,
};

use std::ops::{Add, Sub, Mul, Div, Rem};

type Board = hashbrown::HashMap<Coord<isize>, Chunk>;

#[derive(Debug)]
pub enum SquareView {
    Clicked(u8),
    Unclicked,
    Flagged,
    Penalty,
    Points,
}

pub struct AbsoluteCoord {
    pub chunk: Coord<isize>,
    pub square: Coord<usize>,
}

impl From<AbsoluteCoord> for Coord<isize> {
    fn from(AbsoluteCoord { chunk, square }: AbsoluteCoord) -> Self {
        chunk * Coord::squared(8) + Coord::<isize>::from(square)
    }
}

impl From<Coord<isize>> for AbsoluteCoord {
    fn from(source: Coord<isize>) -> Self {
        AbsoluteCoord {
            chunk:  source.map(|x| x.div_floor(chunk::DIMENSION as isize)),
            square: source.map(|x| x.modulo   (chunk::DIMENSION as isize)).into(),
        }       
    }
}

#[derive(Default)]
pub struct Game {
    pub chunks: Board,
    chunks_won: u64,
    chunks_lost: u64,
}

    
impl Game {
    pub fn new() -> Self { Game::default() }
    pub fn chunks_won(&self) -> u64 { self.chunks_won }
    pub fn chunks_lost(&self) -> u64 { self.chunks_lost }
    pub fn get_chunk(&self, chunk: Coord<isize>) -> Option<&Chunk> { self.chunks.get(&chunk) }
    
    fn allocate_with_surround(&mut self, chunk: Coord<isize>, square: Coord<usize>) {
        use hashbrown::hash_map::Entry::Vacant;

        if let Vacant(entry) = self.chunks.entry(chunk) {
            entry.insert(loop {
                // Ensure first click is not a mine
                let insert = Chunk::with_mines();
                if insert.mines.get(square) { continue; }
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
            let AbsoluteCoord { chunk, square } = world_coord.into();

            self.allocate_with_surround(chunk, square);
            self.calc_neighbors(chunk);
            
            {
                let touched_chunk = self.chunks.get_mut(&chunk).unwrap();
                
                if touched_chunk.clicked.get(square) { continue; }
                
                // Actually click
                touched_chunk.flags.unset(square);
                touched_chunk.clicked.set(square);
                
                if touched_chunk.mines.get(square) {
                    touched_chunk.status = chunk::Status::Lost;
                    self.chunks_lost += 1;
                    return None;
                }
                
                if touched_chunk.is_won() {
                    touched_chunk.status = chunk::Status::Won;
                    self.chunks_won += 1;
                }
            }
            
            let num_neighbors = self
                .chunks
                .get(&chunk)
                .unwrap()
                .neighbors
                .get(square);
            
            if num_neighbors == 0 {
                let fringe = IndexIterSigned::self_and_adjacent(Coord::<isize>::default())
                    .filter(|&offset| offset != Coord::default())
                    .map(|offset| offset + world_coord);
                
                to_click.extend(fringe);
            }
        }
        
        to_click.optionalize()
    }
    
    pub fn toggle_flag(&mut self, world_coord: Coord<isize>) {
        let AbsoluteCoord { chunk, square } = world_coord.into();

        self.allocate_with_surround(chunk, square);
        let chunk = self.chunks.get_mut(&chunk).unwrap();
        
        if !chunk.clicked.get(square) {
            chunk.flags.toggle(square);
        }
        
        if chunk.is_won() {
            chunk.status = chunk::Status::Won;
            self.chunks_won += 1;
        }
    }
    
    fn calc_neighbors(&mut self, coord: Coord<isize>) {
        debug_assert!(
            IndexIterSigned::self_and_adjacent(coord)
                .all(|chunk| self.chunks.contains_key(&chunk))
        );

        let neighbors = {
            let mut canvas = NybbleField::default();
            let center = self.chunks.get(&coord).unwrap();
            
            if center.status != chunk::Status::Enmined { return; }
            
            let surround = IndexIterSigned::self_and_adjacent(coord)
                    .map(|target| self.chunks.get(&target))
                    .collect::<Option<Vec<_>>>()
                    .unwrap();
            
            for square_index in chunk::all_squares() {
                if center.mines.get(square_index) { continue; } // Mine squares have no count

                let square_index_i = Coord::<isize>::from(square_index);
                
                let count = IndexIterSigned::self_and_adjacent(Coord::<isize>::default())
                    .map(|offset| {
                        const DIMENSION_COORD: Coord<isize> = Coord::squared(chunk::DIMENSION as isize);

                        let RENAME_ME = square_index_i
                            .add(offset)
                            .add(DIMENSION_COORD)
                            .div(DIMENSION_COORD);
                        
                        let chunk = RENAME_ME
                            .mul(Coord(3, 1))
                            .sum() as usize;

                        let square: Coord<usize> = square_index_i
                            .add(offset)
                            .add(
                                Coord::squared(2)
                                    .sub(RENAME_ME)
                                    .mul(DIMENSION_COORD)
                            )
                            .rem(DIMENSION_COORD)
                            .into();
                        
                        (chunk, square)
                    })
                    .filter(|&(chunk, square)| surround[chunk].mines.get(square))
                    .count();
                
                debug_assert!(count < std::u8::MAX.into());
                
                canvas.set(square_index, count as u8);
            }
            
            canvas
        };

        let dest = self.chunks.get_mut(&coord).unwrap();
        dest.neighbors = neighbors;
        dest.status = chunk::Status::Neighbored;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn chunk_cascade() {
        let touch_points: [Coord<isize>;5] = [
            Coord( 0,  0),
            Coord( 0, 16),
            Coord( 8, 24),
            Coord(16,  8),
            Coord( 8,  8),
        ];
        
        let mut game = Game::default();
        game.touch(&touch_points);
        
        let active_count = game
            .chunks
            .values()
            .filter(|&chunk| chunk.status != chunk::Status::Enmined)
            .count();
        
        assert_eq!(game.chunks.len(), 25);
        assert_eq!(active_count, 5);
    }
}
