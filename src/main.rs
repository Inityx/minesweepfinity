extern crate ncurses;

use std::env;

mod msgame;

fn main() {
    let mut g = msgame::Game::new();
    g.test_touch();
    
    if !env::args().any(|x| x == "--noprint") {
        g.print();
        ncurses::getch();
    }
}

