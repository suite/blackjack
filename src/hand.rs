use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>
}


impl Hand {
    pub fn get_value(&self) -> (u8, Option<u8>) {
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
