use crate::card::Card;

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn take_card(&mut self) -> Card {
        match self.cards.pop() {
            Some(card) => card,
            None => todo!("Handle out of cards")
        }
    }
}
