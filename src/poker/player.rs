use crate::deck::{ Card };

#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub stack: u32,
    pub player_num: u8,
}

impl Player {
    pub fn new(stack: u32, player_num: u8) -> Self {
        Player { 
            hand: Vec::with_capacity(2), 
            stack: stack,
            player_num,
        }
    }

    pub fn deal_new_hand(&mut self, hand: Vec<Card>) {
        if hand.len() != 2 {
            panic!("Your new hand should be 2 cards. Your input was {} card(s).", hand.len());
        }

        self.hand = hand;
    }
}