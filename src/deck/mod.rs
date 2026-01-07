use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone)]
pub enum Suite {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, EnumIter, Clone)]
pub enum Rank {
    Ace,
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
}
