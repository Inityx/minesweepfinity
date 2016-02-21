extern crate ncurses;

mod msgame;

fn main() {
    let mut g = msgame::Game::new();
    g.test_touch();
    g.print();
    ncurses::getch();
}

