use std::{error::Error, fmt::{write, Display}, io};

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn take_card(&mut self) -> Card {
        match self.cards.pop() {
            Some(card) => card,
            None => todo!("Handle out of cards")
        }
    }
}

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
enum Rank {
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

    fn value(&self) -> (u8, Option<u8>) {
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
struct Card {
    rank: Rank,
    suite: Suite,
}

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
struct Hand {
    cards: Vec<Card>
}

// impl Display for Hand {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // [Card { rank: Ace, suite: Diamond }, Card { rank: Three, suite: Spade }]
//         // A<> 3S
//         todo!("do sum");
//         let mut card_str = String::new();
//         for card in &self.cards {
//             write!(f, "{}", card_str)
//         }
//         Ok(())
        
//     }
// }

impl Hand {
    fn get_value(&self) -> (u8, Option<u8>) {
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
}

pub struct Player {
    name: String,
    balance: u32,
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

impl Player {
    pub fn new() -> Result<Player, &'static str> {
        let mut name = String::new();
        let mut balance = String::new();

        println!("Enter your player name:");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read player name");

        let name = name.trim().to_string();

        println!("How much money you got?");
        io::stdin()
            .read_line(&mut balance)
            .expect("Failed to read balance");

        let balance: u32 = match balance.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse balance")
        };

        println!("Player: {name} Balance: {balance}");

        Ok(Player {
            name,
            balance
        })
    }

    fn request_bet_amount(&self) -> Result<u32, Box<dyn Error>> {
        let mut bet_amount = String::new();

        println!("How much do you want to put on this game? Balance: {}", self.balance);
        io::stdin()
            .read_line(&mut bet_amount)?;
        
        let bet_amount: u32 = bet_amount.trim().parse()?;

        if bet_amount > self.balance {
            return Err("Bet amount exceeds player balance".into());
        }

        Ok(bet_amount)
    }

    fn request_action(&self) -> Result<Action, Box<dyn Error>> {
        let mut action = String::new();
        
        // TODO Need to get available moves
        println!("What would you like to do? [H]it [Sp]lit [D]ouble [S]tand");
        io::stdin()
            .read_line(&mut action)?;

        match action.trim() {
            "H" => Ok(Action::Hit),
            "Sp" => Ok(Action::Split),
            "D" => Ok(Action::Double),
            "S" => Ok(Action::Stand),
            _ => return Err("Unknown action".into())
        }
    }
}

mod card_utils {
    use rand::{seq::SliceRandom, thread_rng};
    use super::{Card, Suite, Rank, Deck};
    
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

impl BlackJack {
    pub fn new(player: Player) -> Result<Self, &'static str> {
        let bet_amount = player.request_bet_amount().expect("bad bet amount");
        let mut deck = card_utils::generate_blackjack_deck(6);

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