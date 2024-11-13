
use std::process;

use blackjack::{game::BlackJack, player::Player};

fn main() {
    let player = Player::new().unwrap_or_else(|err| {
        println!("Could not get player: {err}");
        process::exit(1);
    });

    let mut game = BlackJack::new(player).expect("couldnt create game");

    while game.is_running() { 
        game.take_action();
    }
}
