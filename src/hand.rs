use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{card::{self, Card}, deck::Deck, game::{Action, BlackJack}};


// TODO: make fields private, use ::new to create and init hands
// reduce number of pub struct -> pub field
pub struct Hand {
    cards: Vec<Card>,
    is_dealer: bool,
    bet_value: u32,
    value: (u8, Option<u8>),
    best_value: Option<u8>,
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
            value: (0, None),
            best_value: None,
            deck
        }
    }

    pub fn hit(&mut self) {
        let card = self.deck.borrow_mut().take_card();
        self.cards.push(card);
        self.value = self.value();
        self.best_value = self.best_value(self.value);
    }

    pub fn take_card(&mut self) -> Option<Card> {
        let card = self.cards.pop();
        self.value = self.value(); // could just add? self.value = self.get_updated
        self.best_value = self.best_value(self.value);
        card
    }

    pub fn update_bet_value(&mut self, update: BetValueUpdate) {
        match update {
            BetValueUpdate::Blackjack => self.bet_value *= 2,
            BetValueUpdate::Double => self.bet_value *= 2, // TODO: set to 1.6
            BetValueUpdate::Bust => self.bet_value *= 0, // TODO: set to -1
        }
    }

    pub fn get_bet_value(&self) -> u32 {
        self.bet_value
    }
    
    fn value(&self) -> (u8, Option<u8>) {
        let mut value = 0;
        let mut option_val: Option<(u8 ,u8)> = None;

        for card in &self.cards {
            let (num1, num2) = card.rank.value();
            match num2 {
                Some(num2) => {
                    match option_val {
                        Some((prev1, prev2)) => option_val = Some((prev1+num1, prev2+num2)),
                        None => option_val = Some((num1, num2))
                    }
                },
                None => value += num1
            }
        }

        if let Some((num1, num2)) = option_val {
            return (value+num1, Some(value+num2));  
        }

        (value, None)
    }

    pub fn get_value(&self) -> (u8, Option<u8>) {
        self.value
    }

    fn best_value(&self, value: (u8, Option<u8>) ) -> Option<u8> {
        let (num1, num2) = value;
        let num2 = num2.unwrap_or(22);

        if num1 > 21 && num2 > 21 {
            None
        } else if num2 > 21 {
            Some(num1)
        } else if num1 > 21 {
            Some(num2)
        } else if num2 > num1 {
            Some(num2)
        } else {
            Some(num1)
        }
    }

    pub fn get_best_value(&self) -> Option<u8> {
        self.best_value
    }

    pub fn get_action(&self) -> Action {
        let (num1, num2) = self.value;
        let num2 = num2.unwrap_or(22);

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
            && self.value.1.unwrap_or(0) == 21
    }
}
