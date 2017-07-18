extern crate ncurses;
extern crate rand;

mod game;
mod interface;
mod aux;

use interface::Interface;
use game::Game;
use aux::coord::Coord;

fn main() {
    let mut game: Game = Game::default();
    let mut interface = Interface::new();
    
    game.touch(Coord(0,0));
    interface.play(&mut game);
}
