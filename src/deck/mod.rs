use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::seq::SliceRandom;
use rand::rng;

#[derive(Debug, EnumIter, Clone)]
pub enum Suite {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, EnumIter, Clone)]
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
    Ace,
}

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suite: Suite,
}

impl Card {
    pub fn new(rank: Rank, suite: Suite) -> Self {
        Card { rank, suite }
    }

    pub fn get_value(&self) -> u8 {
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);

        for suite in Suite::iter() {
            for rank in Rank::iter() {
                cards.push(Card { 
                    rank: rank.clone(), 
                    suite: suite.clone(),
                });
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) -> () {
        self.cards.shuffle(&mut rng());
    }
}
