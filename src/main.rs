
use std::process;

use blackjack::{game::BlackJack, player::Player};

/*
TODO:
add some form of rust lint
*/

fn main() {
    let player = Player::new().unwrap_or_else(|err| {
        println!("Could not get player: {err}");
        process::exit(1);
    });

    let mut game = BlackJack::new(player).expect("couldnt create game");

    // new round
    while game.is_running() { 
        // action start at player
        game.take_action();

        // game.take_player_action() // hit, split, double, stand
        // if double/stand -> dealer show card
        // if hit -> get card, hit/stand 

        // if split -> player
            // hit/stand
                // stand
                    // move to next card
                // if hit -> get card, hit/stand

                
            
        // game.take_dealer_action()
        // game.take_player_action()

    }
}
