extern crate ncurses;
extern crate rand;
extern crate core;

mod game;
mod interface;
mod aux;

use interface::Interface;
use game::Game;

fn main() {
    let mut game: Game = Default::default();
    
    game.test_touch();
    
    if std::env::args().any(|arg| arg == "--noprint") {
        game.chunk_debug();
        return;
    }
    
    let mut interface = Interface::new();
    interface.play(&mut game);
}
