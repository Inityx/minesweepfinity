extern crate ncurses;

mod msgame;
use msgame::*;

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
	let mut g = Game::new();
	g.print();
	ncurses::getch();
}

