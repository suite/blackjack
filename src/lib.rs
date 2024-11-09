use std::{error::Error, fmt::Display, io};

pub struct BlackJack {
    player: Player,
    turn: Turn,
    bet_amount: u32,
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

        let balance = match balance.trim().parse() {
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

impl BlackJack {
    pub fn new(player: Player) -> Result<Self, &'static str> {
        let bet_amount = player.request_bet_amount().expect("bad bet amount");

        Ok(BlackJack {
            player,
            turn: Turn::Player,
            bet_amount
        })
    }

    pub fn get_player_action(&self) ->  Result<Action, &'static str>  {
        let action = self.player.request_action().expect("bad action");
        Ok(action)
    }

    pub fn deal_cards(&self) {
        println!("Dealing cards...");
    }

    pub fn take_player_action(&mut self, action: Action) {
        println!("Taking action {action:?}");

        // depending on what we do, we want to change turn
        self.turn = Turn::Dealer
    }

    pub fn take_dealer_action(&mut self) {
        println!("Taking dealer action...");

        // depending on what we do, we want to change turn
        self.turn = Turn::Player
    }

    pub fn is_running(&self) -> bool{
        return true;
    }

    pub fn get_turn(&self) -> &Turn {
        &self.turn
    }
}