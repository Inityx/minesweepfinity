extern crate ncurses;

mod msgame;

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
	let mut g = msgame::Game::new();
	g.test_touch();
	g.print();
	ncurses::getch();
}

