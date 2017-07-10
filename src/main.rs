extern crate ncurses;
extern crate rand;

mod game;
mod interface;

use interface::Interface;

fn main() {
    let mut game: game::Game = Default::default();
    
    game.test_touch();
    
    if std::env::args().any(|arg| arg == "--noprint") {
        game.chunk_debug();
        return;
    }
    
    let mut interface = Interface::new();
    interface.play(&mut game);
    ncurses::getch();
}
