mod board;
pub mod chunk;

use game::board::Board;
use game::chunk::SquareView;

use ::aux::coord::Coord;

#[derive(Default)]
pub struct Game {
    board: Board,
    chunks_won: usize,
}

impl Game {
    pub fn test_touch(&mut self) { self.board.touch(&Default::default()) }
    pub fn get_chunks_won(&self) -> usize { self.chunks_won }
    pub fn view_chunk(&self, coord: &Coord<isize>) -> Vec<SquareView> {
        self.board.get_chunk(coord).unwrap().view()
    }
    pub fn chunk_debug(&self) {
        println!(
            "{:?}",
            self.board.get_chunk(&Default::default())
        );
    }
}
