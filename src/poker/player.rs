use crate::deck::{ Card, HandRanking, Rank };
use itertools::Itertools;
use std::collections::HashMap;

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

    pub fn set_new_hand(&mut self, hand: Vec<Card>) -> () {
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

    fn evaluate_hand(hand: &[&Card]) -> HandRanking {
        let mut rank_counts = HashMap::new();
        let mut suit_counts = HashMap::new();

        // Track count of each suite and rank
        for card in hand {
            *rank_counts.entry(card.rank).or_insert(0) += 1;
            *suit_counts.entry(card.suite).or_insert(0) += 1;
        }

        // Is a flush if have 5 of any suite
        let is_flush = suit_counts.values().any(|&c| c == 5);

        // Put all rank values into a list (include duplicates) and sort them
        let mut ranks: Vec<Rank> = hand.iter().map(|c| c.rank).collect();
        ranks.sort();

        let is_straight = ranks
            .windows(2)
            .all(|w| w[1] as i32 == w[0] as i32 + 1)
            || ranks == [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Ace];

        let mut counts: Vec<i32> = rank_counts.values().cloned().collect();
        counts.sort_by(|a, b| b.cmp(a)); // descending

        // Only evaluates 5 card hands
        match (is_straight, is_flush, counts.as_slice()) {
            (true, true, _) if ranks.contains(&Rank::Ace) && ranks.contains(&Rank::King) =>
                HandRanking::RoyalFlush,

            (true, true, _) =>
                HandRanking::StraightFlush,

            (_, _, [4, 1]) =>
                HandRanking::FourOfAKind,

            (_, _, [3, 2]) =>
                HandRanking::FullHouse,

            (_, true, _) =>
                HandRanking::Flush,

            (true, _, _) =>
                HandRanking::Straight,

            (_, _, [3, 1, 1]) =>
                HandRanking::ThreeOfAKind,

            (_, _, [2, 2, 1]) =>
                HandRanking::TwoPair,

            (_, _, [2, 1, 1, 1]) =>
                HandRanking::OnePair,

            _ =>
                match ranks.iter().max().unwrap() {
                    Rank::Two => HandRanking::TwoHigh,
                    Rank::Three => HandRanking::ThreeHigh,
                    Rank::Four => HandRanking::FourHigh,
                    Rank::Five => HandRanking::FiveHigh,
                    Rank::Six => HandRanking::SixHigh,
                    Rank::Seven => HandRanking::SevenHigh,
                    Rank::Eight => HandRanking::EightHigh,
                    Rank::Nine => HandRanking::NineHigh,
                    Rank::Ten => HandRanking::TenHigh,
                    Rank::Jack => HandRanking::JackHigh,
                    Rank::Queen => HandRanking::QueenHigh,
                    Rank::King => HandRanking::KingHigh,
                    Rank::Ace => HandRanking::AceHigh,
                },
        }
    }

    pub fn get_best_hand_value(&self, community_cards: &Vec<Card>) -> Result<HandRanking, String> {
        let mut all_cards: Vec<&Card> = Vec::new();
        all_cards.extend(community_cards.iter());
        all_cards.extend(self.hand.iter());

        // Find all possible hands
        let best_hand = all_cards
            .into_iter()
            .combinations(5)
            .map(|hand| Self::evaluate_hand(hand.as_slice())) // evaluate each possible 5 card hand
            .max()
            .unwrap();

        Ok(best_hand)
    }
}