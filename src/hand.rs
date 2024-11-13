use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{card::{self, Card}, deck::Deck, game::{Action, BlackJack}};


// TODO: make fields private, use ::new to create and init hands
// reduce number of pub struct -> pub field
pub struct Hand {
    cards: Vec<Card>,
    is_dealer: bool,
    bet_value: u32,
    value: (u8, u8),
    best_value: u8,
    deck: Rc<RefCell<Deck>>
}

pub enum BetValueUpdate {
    Blackjack,
    Bust,
    Double
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hand = String::new();
        for card in &self.cards {
            hand.push_str(&card.to_string()); // TODO: clean this up
        }
        write!(f, "{hand}")
    }
}


impl Hand {
   pub fn new(is_dealer: bool, bet_amount: Option<u32>, deck: Rc<RefCell<Deck>>, split_card: Option<Card>) -> Self {
        let mut cards = vec![];
        if let Some(card) = split_card {
            cards.push(card);
        }
        Hand {
            cards,
            is_dealer,
            bet_value: bet_amount.unwrap_or(0),
            value: (0, 0),
            best_value: 0,
            deck
        }
    }

    pub fn hit(&mut self) {
        let card = self.deck.borrow_mut().take_card();
        
        self.update_values(&card, true);
        
        self.cards.push(card);
    }

    pub fn take_card(&mut self) -> Option<Card> {
        let card = self.cards.pop();

        if let Some(ref card) = card {
            self.update_values(card, false);
        }
        
        card
    }

    fn update_values(&mut self, card: &Card, should_add: bool) {
        if should_add {
            self.value.0 += card.rank.value().0;
            self.value.1 += card.rank.value().1.unwrap_or(card.rank.value().0);    
        } else {
            self.value.0 -= card.rank.value().0;
            self.value.1 -= card.rank.value().1.unwrap_or(card.rank.value().0);
        }

        if self.value.0 > 21 && self.value.1 > 21 {
            self.best_value = 0;
            return;
        }
        
        if self.value.0 <= 21 {
            self.best_value = self.value.0;
        }

        if self.value.1 <= 21 && self.value.1 > self.best_value {
            self.best_value = self.value.1;
        }
    }

    pub fn update_bet_value(&mut self, update: BetValueUpdate) {
        match update {
            BetValueUpdate::Blackjack => self.bet_value *= 2,
            BetValueUpdate::Double => self.bet_value *= 2, // TODO: set to 1.6
            BetValueUpdate::Bust => self.bet_value *= 0, // TODO: set to -1
        }
    }

    pub fn bet_value(&self) -> u32 {
        self.bet_value
    }

    pub fn value(&self) -> (u8, u8) {
        self.value
    }

    pub fn best_value(&self) -> u8 {
        self.best_value
    }

    pub fn get_action(&self) -> Action {
        let (num1, num2) = self.value;

        if num1 > 21 && num2 > 21 {
            return Action::Bust;
        } else if (num1 >= 17 && num1 <= 21) || (num2 >= 17 && num2 <= 21) {
            if self.is_dealer || num1 == 21 || num2 == 21 {
                return Action::Stand;
            }
        }

        Action::DoNothing
    }

    pub fn can_split(&self) -> bool {
        self.cards.len() == 2 
            && self.cards[0].rank.value() == self.cards[1].rank.value()
    }

    pub fn can_double(&self) -> bool {
        self.cards.len() == 2
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 
            && self.value.1 == 21
    }
}
