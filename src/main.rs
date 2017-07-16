extern crate ncurses;
extern crate rand;
extern crate core;

mod game;
mod interface;
mod aux;

use interface::Interface;
use game::Game;
use aux::coord::Coord;

fn main() {
    let mut game: Game = Game::default();
    
    game.touch(Coord(0,0));
    
    if std::env::args().any(|arg| arg == "--noprint") {
        game.chunk_debug(Coord(0,0));
        return;
    }
    
    let mut interface = Interface::new();
    interface.play(&mut game);
}
