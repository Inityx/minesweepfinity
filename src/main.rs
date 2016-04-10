extern crate ncurses;

use std::env;

mod msgame;

fn main() {
    let mut g = msgame::Game::new();
    g.test_touch();
    
    if env::args().any(|x| x == "--noprint") {
        g.chunk_debug();
    } else {
        g.init_printer();
        g.print();
        ncurses::getch();
    }
}

