use crate::{card, deck::Deck, hand::Hand, player::Player};

pub struct BlackJack {
    player: Player,
    turn: Turn,
    bet_amount: u32,
    deck: Deck,
    is_running: bool,

    current_hand_index: usize, // TODO: move to ref?
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

pub enum Turn {
    Player,
    Dealer
}


impl BlackJack {
    pub fn new(mut player: Player) -> Result<Self, &'static str> {
        let bet_amount = player.request_bet_amount().expect("bad bet amount");
        let mut deck = card::card_utils::generate_blackjack_deck(6);

        // dealer starts with 1 card
        // player starts with two cards
        let mut dealer_hand = Hand { cards: vec![], is_dealer: true, bet_value: 0 };
        let mut player_hands = vec![ Hand { cards: vec![], is_dealer: false, bet_value: bet_amount }];
        let mut init_player_hand =  player_hands.get_mut(0).unwrap();

        deck.hit(&mut dealer_hand);
        deck.hit(&mut init_player_hand);
        deck.hit(&mut init_player_hand);

        let mut start_turn = Turn::Player;
        // TODO: blackjack check
        if init_player_hand.is_blackjack() {
            println!("Blackjack!");
            init_player_hand.bet_value *= 2; // TODO
            start_turn = Turn::Dealer;
        }
        
        println!("Dealer shows {} value: {:?}", dealer_hand, dealer_hand.value());
        println!("Player shows {} value: {:?}", player_hands.get(0).unwrap(), player_hands.get(0).unwrap().value());
        
        Ok(BlackJack {
            player,
            turn: start_turn,
            bet_amount,
            deck,
            is_running: true,
            current_hand_index: 0,
            player_hands,
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
        println!("Taking action {action:?}");
        match action {
            Action::Hit => {
                let curr_hand = &mut self.player_hands[self.current_hand_index];
                self.deck.hit(curr_hand);
                let next_action = curr_hand.get_action();
            
                // blackjack test
                // needs to modify hand value
                if curr_hand.is_blackjack() {
                        // blackjack
                        println!("Blackjack!");
                        curr_hand.bet_value *= 2; // TODO: this should be 1.5
                }
               
                // bust or 21, move to next hand
                self.take_player_action(next_action);
            },
            Action::Split => {
                // split cards (check balance)
                match self.player.withdraw_balance(self.bet_amount) {
                    Ok(_) => {
                        let curr_hand = &mut self.player_hands[self.current_hand_index];

                        if !curr_hand.can_split() { 
                            println!("Can't split");
                            return;
                        };

                        let split_card = curr_hand.cards.pop().unwrap();
                        self.player_hands.insert(self.current_hand_index+1, 
                            Hand {
                            cards: vec![split_card],
                            is_dealer: false,
                            bet_value: self.bet_amount
                        });
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
                        
                        if curr_hand.cards.len() != 2 {
                            println!("Can't double");
                            return;
                        }
                        
                        // needs to modify hand value
                        curr_hand.bet_value = curr_hand.bet_value * 2;
                        self.deck.hit(curr_hand);

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
                    self.player_hands[self.current_hand_index].bet_value = 0;
                } 

                if self.player_hands.len() == 1 || self.current_hand_index == self.player_hands.len()-1 {
                    self.turn = Turn::Dealer
                } else {
                    self.current_hand_index += 1;
                }
            },
            Action::DoNothing => {}
        }
    }

    fn take_dealer_action(&mut self) {
        self.deck.hit(&mut self.dealer_hand);

        match self.dealer_hand.get_action() {
            Action::Bust | Action::Stand  => {
                // TODO: implement final game check
                self.is_running = false;
            },
            _ => {}
        }
    }

    pub fn is_running(&self) -> bool{
        return self.is_running;
    }
}