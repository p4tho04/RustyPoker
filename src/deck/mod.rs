use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::seq::SliceRandom;
use rand::rng;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suite {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, EnumIter, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRanking {
    TwoHigh,
    ThreeHigh,
    FourHigh,
    FiveHigh,
    SixHigh,
    SevenHigh,
    EightHigh,
    NineHigh,
    TenHigh,
    JackHigh,
    QueenHigh,
    KingHigh,
    AceHigh,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suite: Suite,
}

impl Card {
    pub fn new(rank: Rank, suite: Suite) -> Self {
        Card { rank, suite }
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
        self.cards.shuffle(&mut rng())
    }

    pub fn remove_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}
