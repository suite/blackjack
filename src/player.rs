use std::{error::Error, io};

use crate::game::Action;

pub struct Player {
    name: String,
    balance: u32,
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

    pub fn request_bet_amount(&mut self) -> Result<u32, Box<dyn Error>> {
        let mut bet_amount = String::new();

        println!("How much do you want to put on this game? Balance: {}", self.balance);
        io::stdin()
            .read_line(&mut bet_amount)?;
        
        let bet_amount: u32 = bet_amount.trim().parse()?;

        self.withdraw_balance(bet_amount)?;

        Ok(bet_amount)
    }

    pub fn request_action(&self) -> Result<Action, Box<dyn Error>> {
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

    // TODO: move some errors, Result<(), ()> -> no, better errors? custom err type
    pub fn withdraw_balance(&mut self, amount: u32) -> Result<u32, &'static str> {
        if amount > self.balance {
            Err("Not enough money")
        } else {
            Ok(amount)
        }
    }

    pub fn deposit_balance(&mut self, amount: u32) {
        self.balance += amount;
    }
}
