//extern crate ncurses;
//use ncurses::*;

mod msgame;

fn main() {
	let mut w = msgame::World::new();
	w.chunk_create(0,0);
}

