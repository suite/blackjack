use crate::{card::Card, hand::Hand};

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    fn take_card(&mut self) -> Card {
        match self.cards.pop() {
            Some(card) => card,
            None => todo!("Handle out of cards")
        }
    }

    pub fn hit(&mut self, hand: &mut Hand) {
        hand.cards.push(self.take_card());
        let who = if hand.is_dealer { "Dealer" } else { "Player "};
        // TODO: Print players name?
        println!("{} hit and got {} value {:?}", who, hand, hand.get_value());
    }
}
