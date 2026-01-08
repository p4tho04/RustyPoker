mod player;
use player:: { Player };
use crate::deck::{ Deck, Card };

const MAX_PLAYERS: usize = 10;
const MIN_PLAYERS: usize = 2;

#[derive(Debug)]
pub enum Stage {
    Flop,
    Turn,
    River,
}

#[derive(Debug)]
pub struct PokerGame {
    pub deck: Deck,
    pub total_players: usize,
    pub players: Vec<Player>,
    pub pot: i32,
    pub current_stage: Stage,
    pub current_round_bet: i32,
}

impl PokerGame {
    pub fn new(total_players: usize, start_stack: i32) -> Self {
        if  total_players < MIN_PLAYERS || total_players > MAX_PLAYERS {
            panic!("Can only have between {} and {} players. Your input had {} player(s).", MIN_PLAYERS, MAX_PLAYERS, total_players);
        }

        let mut players = Vec::with_capacity(total_players);
        let deck = Deck::new();

        for player_num in 1..=total_players {
            let player: Player = Player::new(
                start_stack,
                player_num as i8,
            );

            players.push(player);
        }

        PokerGame {
            deck: deck,
            total_players: total_players,
            players: players,
            pot: 0,
            current_stage: Stage::Flop,
            current_round_bet: 0,
        }
    }

    pub fn shuffle_deck(&mut self) -> () {
        self.deck.shuffle();
    }

    pub fn reset_deck(&mut self) -> () {
        let new_deck = Deck::new();

        self.deck = new_deck;

        self.shuffle_deck();
    }

    pub fn deal_cards(&mut self) -> () {
        for player in self.players.iter_mut() {
            let card1: Card = self.deck.remove_card();
            let card2: Card = self.deck.remove_card();

            let hand = vec![card1, card2];

            player.deal_new_hand(hand);
        }
    }

    pub fn print_players(&self) -> () {
        for player in self.players.iter() {
            println!("{:?}", player);
        }
    }

    pub fn print_cards(&self) -> () {
        for card in self.deck.cards.iter() {
            println!("{:?}", card);
        }
    }
}