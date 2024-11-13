use std::{cell::RefCell, rc::Rc};

use crate::{deck::Deck, hand::{BetValueUpdate, Hand}, player::Player};

pub struct BlackJack {
    player: Player,
    turn: Turn,
    bet_amount: f32,
    deck: Rc<RefCell<Deck>>,
    is_running: bool,
    current_hand_index: usize,
    player_hands: Vec<Hand>, 
    dealer_hand: Hand
}

#[derive(Debug)]
pub enum Action {
    Hit,
    Split,
    Double,
    Stand,
    Bust,
    DoNothing // prob better way to handle
}

enum Turn {
    Player,
    Dealer
}

impl BlackJack {
    pub fn new(mut player: Player) -> Result<Self, &'static str> {
        let bet_amount = player.request_bet_amount().expect("bad bet amount");
        let deck = Rc::new(RefCell::new(Deck::new(6)));

        let mut dealer_hand = Hand::new(
                true, 
                0.0, 
                Rc::clone(&deck));
        let mut player_hand = Hand::new(
                false, 
                bet_amount, 
                Rc::clone(&deck));

        dealer_hand.hit();
        player_hand.hit();
        player_hand.hit();

        let mut start_turn = Turn::Player;
        if player_hand.is_blackjack() {
            println!("Blackjack!");

            player_hand.update_bet_value(BetValueUpdate::Blackjack);
            start_turn = Turn::Dealer;
        }
        
        println!("Dealer shows {} value: {:?}", dealer_hand, dealer_hand.value());
        println!("Player shows {} value: {:?}", player_hand, player_hand.value());
        
        Ok(BlackJack {
            player,
            turn: start_turn,
            bet_amount,
            deck,
            is_running: true,
            current_hand_index: 0,
            player_hands: vec![player_hand],
            dealer_hand,
        })
    }

    pub fn take_action(&mut self) {
        match self.turn {
            Turn::Player => {
                let action = self.player.request_action().expect("bad action");
                self.take_player_action(action); 
            },
            Turn::Dealer => self.take_dealer_action(),
        }
    }

    fn take_player_action(&mut self, action: Action) {
        match action {
            Action::Hit => {
                let curr_hand = &mut self.player_hands[self.current_hand_index];
                curr_hand.hit();
                let next_action = curr_hand.get_action();
            
                println!("Player hit: {} value: {:?}", curr_hand, curr_hand.value());
               
                if curr_hand.is_blackjack() {
                    println!("Blackjack!");
                    curr_hand.update_bet_value(BetValueUpdate::Blackjack);
                }

                // bust or 21, move to next hand
                self.take_player_action(next_action);
            },
            Action::Split => {
                match self.player.withdraw_balance(self.bet_amount) {
                    Ok(_) => {
                        let curr_hand = &mut self.player_hands[self.current_hand_index];

                        if !curr_hand.can_split() { 
                            println!("Can't split");
                            return;
                        };

                        let split_card = curr_hand.take_card().unwrap();
                        let mut new_hand = Hand::new(
                            false, 
                            self.bet_amount, 
                            Rc::clone(&self.deck),
                        );

                        new_hand.push_card(split_card);
                        self.player_hands.insert(self.current_hand_index+1, new_hand);
                    },
                    Err(err) => {
                        println!("Could not withdraw balance {err}");
                        return;
                    }
                }
            },
            Action::Double => {
                match self.player.withdraw_balance(self.bet_amount) {
                    Ok(_) => {
                        let curr_hand = &mut self.player_hands[self.current_hand_index];
                        
                        if !curr_hand.can_double() {
                            println!("Can't double");
                            return;
                        }
                        
                        // needs to modify hand value
                        curr_hand.hit();
                        curr_hand.update_bet_value(BetValueUpdate::Double);

                        let next_action = match curr_hand.get_action() {
                            Action::Bust => Action::Bust,
                            _ => Action::Stand
                        };

                        self.take_player_action(next_action);
                    },
                    Err(err) => {
                        println!("Could not withdraw balance {err}");
                        return;
                    }
                }
            },
            Action::Stand | Action::Bust => {
                if let Action::Bust = action {
                    self.player_hands[self.current_hand_index]
                        .update_bet_value(BetValueUpdate::Bust);
                } 

                if self.current_hand_index == self.player_hands.len()-1 {
                    self.turn = Turn::Dealer
                } else {
                    self.current_hand_index += 1;
                }
            },
            Action::DoNothing => {}
        }
    }

    fn take_dealer_action(&mut self) {
        self.dealer_hand.hit();

        println!("Dealer hit: {} value: {:?}", self.dealer_hand, self.dealer_hand.value());

        match self.dealer_hand.get_action() {
            Action::Bust | Action::Stand  => {
                self.finish();
            },
            _ => {}
        }
    }

    pub fn is_running(&self) -> bool{
        return self.is_running;
    }

    fn finish(&mut self) {
        self.is_running = false;

        let (won, push, lost) = self.calculate_winnings();

        self.player.deposit_balance(won + push);

        println!("Won: {won} Push: {push} Lost: {lost}");
    }

    fn calculate_winnings(&self) -> (f32, f32, f32) {
        let mut won: f32 = 0.0;
        let mut push: f32 = 0.0;
        let mut lost: f32 = 0.0;

        let dealer_best = self.dealer_hand.best_value();
        for hand in &self.player_hands {
            let player_best = hand.best_value();
            let bet_value = hand.bet_value();
            if player_best > dealer_best {
                won += bet_value * 2.0;
            } else if player_best == dealer_best {
                if bet_value.is_sign_positive() {
                    push += bet_value
                } else {
                    lost += bet_value;
                }     
            } else {
                // lost
                lost += bet_value;
            }
        }

        (won, push, lost)
    }
}