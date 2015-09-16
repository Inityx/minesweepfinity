extern crate ncurses;
use ncurses::*;

mod msgame;

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
	let mut w = msgame::World::new();
	w.touch(0,0);

	initscr();
	printw("Hello, world");
	refresh();
	getch();

	endwin();
}

