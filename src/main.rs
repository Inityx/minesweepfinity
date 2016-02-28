extern crate ncurses;

use std::env;

mod msgame;

fn main() {
    let mut g = msgame::Game::new();
    g.test_touch();
    
    if !env::args().any(|x| x == "--noprint") {
        g.init_printer();
        g.print();
        ncurses::getch();
    } else {
        g.chunk_debug();
    }
}

