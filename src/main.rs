
use std::{io::Split, process};

use blackjack::{BlackJack,Player,Turn};

fn main() {
    // enter name, enter balance
    let player = Player::new().unwrap_or_else(|err| {
        println!("Could not get player: {err}");
        process::exit(1);
    });

    // do we want this to be mutable?
    let mut game = BlackJack::new(player).expect("couldnt create game");

    game.deal_cards();

    // new round
    while game.is_running() { 
        // action start at player
        let turn = game.get_turn(); // enum, Player, Dealer

        match turn {
            Turn::Player => {
                let action = game.get_player_action().expect("should have player action"); 
                game.take_player_action(action); // could be game.take_action()
            },
            Turn::Dealer => game.take_dealer_action() // show card or hit
        }

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
