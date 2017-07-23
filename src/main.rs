extern crate ncurses;
extern crate rand;

mod game;
mod interface;
mod aux;

use interface::Interface;
use game::Game;

fn main() {
    let mut game: Game = Game::default();
    let mut interface = Interface::new();
    
    interface.play(&mut game);
}
