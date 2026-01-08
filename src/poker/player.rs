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

    pub fn fold(&mut self) -> () {
        self.active = false;
    }

    pub fn check(&self, current_round_bet: i32) -> Result<i32, String> {
        if self.wager < current_round_bet {
            return Err("You have to call or raise because your current wager is less than the current bet".to_string());
        }

        Ok(0) // add to pot
    }

    pub fn call(&mut self, current_round_bet: i32) -> Result<i32, String> {
        let wager_bet_difference: i32 = current_round_bet - self.wager;

        if wager_bet_difference > self.stack { // can't match current bet size
            return Err("You don't have enough in your stack to call!".to_string());
        }

        self.stack -= wager_bet_difference;
        self.wager += wager_bet_difference;

        Ok(wager_bet_difference) // add to pot
    }
    
    pub fn raise(&mut self, raise_amnt: i32, current_round_bet: i32) -> Result<i32, String> {
        let expected_bet_after_raise = current_round_bet + raise_amnt;
        let needed_amnt = expected_bet_after_raise - self.wager;

        if raise_amnt > self.stack {
            return Err("You raised more than you have in your stack!".to_string());
        } else if raise_amnt < 1 {
            return Err("You can't raise less than $1!".to_string());
        } else if self.stack + raise_amnt <= current_round_bet {
            return Err("You need to raise more to beat the current round bet!".to_string());
        } else if self.stack < needed_amnt {
            return Err("You don't have enough money to raise this much!".to_string());
        }

        self.stack -= needed_amnt;
        self.wager += needed_amnt;

        Ok(needed_amnt) // add to pot
    }
}