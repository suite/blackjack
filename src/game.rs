use crate::{card, deck::Deck, hand::Hand, player::Player};

pub struct BlackJack {
    player: Player,
    turn: Turn,
    bet_amount: u32,
    deck: Deck,

    current_hand_index: usize, // TODO: move to ref?
    player_hands: Vec<Hand>, 
    dealer_hand: Hand
}

#[derive(Debug)]
pub enum Action {
    Hit,
    Split,
    Double,
    Stand
}

pub enum Turn {
    Player,
    Dealer
}


impl BlackJack {
    pub fn new(player: Player) -> Result<Self, &'static str> {
        let bet_amount = player.request_bet_amount().expect("bad bet amount");
        let mut deck = card::card_utils::generate_blackjack_deck(6);

        // player starts with two cards
        // dealer starts with 1 card
        let player_hands = vec![ Hand { cards: vec![deck.take_card(), deck.take_card()] }];
        let dealer_hand = Hand { cards: vec![deck.take_card()] };

        println!("Dealer shows {:?}", dealer_hand);
        println!("Player shows {:?}", player_hands);

        Ok(BlackJack {
            player,
            turn: Turn::Player,
            bet_amount,
            deck,
            current_hand_index: 0,
            player_hands,
            dealer_hand
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
                // add card to current hand 
                // self.get_current_hand() ?
                let curr_hand = self.player_hands.get_mut(self.current_hand_index).unwrap();
                let curr_hand_cards = &mut curr_hand.cards;

                curr_hand_cards.push(self.deck.take_card());

                // print hand
                println!("{curr_hand_cards:?}");
                println!("value: {:?}", curr_hand.get_value());

                // bust or 21, move to next hand
            },
            Action::Split => {
                // split cards (check balance)
            },
            Action::Double => {
                // double bet (check bet_amount and balance) => stand
                // not available if split (not always true)
            },
            Action::Stand => {
                // Move to next hand if split, otherwise dealer show 
            },
        }

        // depending on what we do, we want to change turn
        // self.turn = Turn::Dealer
    }

    fn take_dealer_action(&mut self) {
        println!("Taking dealer action...");

        // depending on what we do, we want to change turn
        self.turn = Turn::Player
    }

    pub fn is_running(&self) -> bool{
        return true;
    }
}