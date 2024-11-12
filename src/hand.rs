use std::fmt::Display;

use crate::{card::Card, game::Action};


// TODO: make fields private, use ::new to create and init hands
// reduce number of pub struct -> pub field
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub is_dealer: bool,
    pub bet_value: u32,
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
    // TODO: cache value?
    pub fn value(&self) -> (u8, Option<u8>) {
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

    // TODO: cache value?
    pub fn best_value(&self) -> Option<u8> {
        let (num1, num2) = self.value();
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

    pub fn get_action(&self) -> Action {
        let (num1, num2) = self.value();
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

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 
            && self.value().1.unwrap_or(0) == 21
    }
}
