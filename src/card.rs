#[derive(Debug, Clone, Copy)]
enum Suite {
    Heart,
    Diamond,
    Club,
    Spade
}

impl Suite {
    fn iterator() -> impl Iterator<Item = Self> {
        [Suite::Heart, Suite::Diamond, Suite::Club, Suite::Spade].iter().copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Rank {
    fn iterator() -> impl Iterator<Item = Self> {
        [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace].iter().copied()
    }

    pub fn value(&self) -> (u8, Option<u8>) {
        match &self {
            Rank::Two => (2, None),
            Rank::Three => (3, None),
            Rank::Four => (4, None),
            Rank::Five => (5, None),
            Rank::Six => (6, None),
            Rank::Seven => (7, None),
            Rank::Eight => (8, None),
            Rank::Nine => (9, None),
            Rank::Ten => (10, None),
            Rank::Jack => (10, None),
            Rank::Queen => (10, None),
            Rank::King => (10, None),
            Rank::Ace => (1, Some(11)),
        }
    } 
}

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    suite: Suite,
}

impl Card {
    pub fn to_string(&self) -> String {
        format!("{:?} of {:?},", self.rank, self.suite) // TODO: need better format
    }
}

// TODO: move to deck?
pub mod card_utils {
    use rand::{seq::SliceRandom, thread_rng};

    use crate::deck::Deck;

    use super::{Card, Suite, Rank};
    
    fn generate_deck() -> Deck {
        let mut cards = Vec::new();
        
        for rank in Rank::iterator() {
            for suite in Suite::iterator() {
                cards.push(Card {
                    rank,
                    suite
                });
            }
        }

        Deck {
            cards
        }
    }

    pub fn generate_blackjack_deck(num_decks: u32) -> Deck {
        let mut cards = Vec::new();

        for _ in 0..num_decks {
            cards.append(&mut generate_deck().cards);
        }

        cards.shuffle(&mut thread_rng()); 

        Deck {
            cards
        }
    }
}


