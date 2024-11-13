#[derive(Debug, Clone, Copy)]
pub enum Suite {
    Heart,
    Diamond,
    Club,
    Spade
}

impl Suite {
    pub fn iterator() -> impl Iterator<Item = Self> {
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
    pub fn iterator() -> impl Iterator<Item = Self> {
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
    pub suite: Suite,
}

impl Card {
    pub fn to_string(&self) -> String {
        format!("{:?} of {:?},", self.rank, self.suite) // TODO: need better format
    }
}