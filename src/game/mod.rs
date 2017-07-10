mod board;
mod chunk;

use game::board::Board;

#[derive(Default)]
pub struct Game {
    board: Board,
    chunks_won: usize,
}

impl Game {    
    pub fn test_touch(&mut self) {
        self.board.touch(0,0);
    }

    pub fn chunk_debug(&self) {
        println!(
            "{:?}",
            self.board.get_chunk(0,0)
        );
    }
    
    pub fn get_chunks_won(&self) -> usize {
        self.chunks_won
    }
}
