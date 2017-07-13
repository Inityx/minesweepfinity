mod board;
mod chunk;

use game::board::Board;

use ::aux::coord::Coord;

#[derive(Debug)]
pub enum SquareView {
    Unclicked {
        flag: bool,
        mine: bool
    },
    Clicked(u8),
}

#[derive(Default)]
pub struct Game {
    board: Board,
    chunks_won: usize,
}

impl Game {
    pub fn test_touch(&mut self) { self.board.touch(&Coord(0,0)) }
    pub fn get_chunks_won(&self) -> usize { self.chunks_won }
    pub fn view_chunk(&self, coord: &Coord<isize>) -> Vec<SquareView> {
        self.board.get_chunk(coord).unwrap().view()
    }
    pub fn chunk_debug(&self) {
        let coord = Coord(0,0);
        if let Some(chunk) = self.board.get_chunk(&coord) {
            println!("{:?}", chunk);
        } else {
            println!("No chunk at {:?}", coord);
        }
    }
}
