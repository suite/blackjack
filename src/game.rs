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
    pub fn new(player: Player) -> Result<Self, &'static str> {
        let bet_amount = player.request_bet_amount().expect("bad bet amount");
        let mut deck = card::card_utils::generate_blackjack_deck(6);

        // dealer starts with 1 card
        // player starts with two cards
        let mut dealer_hand = Hand { cards: vec![], is_dealer: true };
        let mut player_hands = vec![ Hand { cards: vec![], is_dealer: false }];
        let mut init_player_hand =  player_hands.get_mut(0).unwrap();

        deck.hit(&mut dealer_hand);
        deck.hit(&mut init_player_hand);
        deck.hit(&mut init_player_hand);

        // TODO: blackjack check
        
        println!("Dealer shows {} value: {:?}", dealer_hand, dealer_hand.get_value());
        println!("Player shows {} value: {:?}", player_hands.get(0).unwrap(), player_hands.get(0).unwrap().get_value());
        
        Ok(BlackJack {
            player,
            turn: Turn::Player,
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
                let next_action = {
                    let curr_hand = self.player_hands
                        .get_mut(self.current_hand_index)
                        .unwrap();
                    self.deck.hit(curr_hand);

                    curr_hand.get_action()
                };
               
                // bust or 21, move to next hand
                self.take_player_action(next_action);
            },
            Action::Split => {
                // split cards (check balance)
                match self.player.withdraw_balance(self.bet_amount) {
                    Ok(_) => {},
                    Err(_) => {}
                }
            },
            Action::Double => {
                // double bet (check bet_amount and balance) => stand
                // not available if split (not always true)
                match self.player.withdraw_balance(self.bet_amount) {
                    Ok(_) => {},
                    Err(_) => {}
                }
                // hit one more time on deck, no more hits allowed
            },
            Action::Stand | Action::Bust => {
                // Move to next hand if split, otherwise dealer show 
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