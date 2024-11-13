use rand::{seq::SliceRandom, thread_rng};
use crate::card::{Card, Rank, Suite};

pub struct Deck {
    cards: Vec<Card>
}

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

impl Deck {
    pub fn new(num_decks: u32) -> Self {
        let mut cards = Vec::new();

        for _ in 0..num_decks {
            cards.append(&mut generate_deck().cards);
        }

        cards.shuffle(&mut thread_rng()); 

        Deck {
            cards
        }
    }
    
    pub fn take_card(&mut self) -> Card {
        match self.cards.pop() {
            Some(card) => card,
            None => todo!("Handle out of cards")
        }
    }
}
