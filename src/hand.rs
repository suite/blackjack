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

    pub fn get_action(&self) -> Action {
        let hand_value = self.value();
        let option_val = hand_value.1.unwrap_or(22);

        if hand_value.0 > 21 && option_val > 21 {
            return Action::Bust;
        } else if (hand_value.0 >= 17 && hand_value.0 <= 21) || (option_val >= 17 && option_val <= 21) {
            if self.is_dealer || hand_value.0 == 21 || option_val == 21 {
                // TODO: add blackjack
                if self.cards.len() == 2 && (hand_value.0 == 21 || option_val == 21) {
                    // mark as blackjack or return blackjack?
                }
                return Action::Stand;
            }
        }

        Action::DoNothing
    }

    pub fn can_split(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].rank.value() == self.cards[1].rank.value()
    }
}
