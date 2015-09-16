//extern crate ncurses;
//use ncurses::*;

mod msgame;

fn main() {
	let mut w = msgame::World::new();
	w.touch(0,0);
}

