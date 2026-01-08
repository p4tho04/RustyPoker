use crate::deck::{ Card };

#[derive(Debug)]
pub struct Player {
    pub active: bool,
    pub hand: Vec<Card>,
    pub stack: i32,
    pub wager: i32,
    pub player_num: i8,
}

impl Player {
    pub fn new(stack: i32, player_num: i8) -> Self {
        Player { 
            active: true,
            hand: Vec::with_capacity(2), 
            stack: stack,
            wager: 0 as i32,
            player_num,
        }
    }

    pub fn deal_new_hand(&mut self, hand: Vec<Card>) -> () {
        if hand.len() != 2 {
            panic!("Your new hand should be 2 cards. Your input was {} card(s).", hand.len());
        }

        self.hand = hand;
    }

    pub fn raise(&mut self, raise_amnt: i32) -> () {
        if raise_amnt > self.stack {
            panic!("You raised too much! You only have ${} in your stack, but you raised ${}.", self.stack, raise_amnt);
        } else if raise_amnt < 1 {
            panic!("You can't raise less than $1. You raised ${}", raise_amnt);
        }

        self.stack -= raise_amnt;
        self.wager += raise_amnt;
    }

    pub fn call(&mut self, current_bet: i32) {
        let wager_bet_difference: i32 = current_bet - self.wager;

        if wager_bet_difference > self.stack { // can't match current bet size
            panic!("You do not have enough to call! You only have ${} in your stack, you need at least ${} to call.", self.stack, wager_bet_difference);
        }

        self.stack -= wager_bet_difference;
        self.wager += wager_bet_difference;
    }

    pub fn check(&self, current_bet: i32) -> () {
        if self.wager < current_bet {
            panic!("You have to bet because your current wager (${}) is less than the current bet (${}).", self.wager, current_bet);
        }
    }

    pub fn fold(&mut self) -> () {
        self.active = false;
    }
}