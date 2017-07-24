extern crate ncurses;
extern crate rand;

mod game;
mod interface;
mod aux;

use interface::Interface;
use game::Game;

fn main() {
    Interface::new().play(Game::default());
}
