#![feature(associated_type_defaults)]

mod game;
mod interface;
mod aux;

use self::{
    interface::Interface,
    game::Game,
};

fn main() {
    Interface::new().play(Game::new());
}
