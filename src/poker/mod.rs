mod player;
use player:: { Player };
use crate::deck::{ Deck, Card };
use std::io;

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
    pub community_cards: Vec<Card>,
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
        let community_cards = Vec::with_capacity(5);

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
            community_cards: community_cards,
            current_stage: Stage::Flop,
            current_round_bet: 0,
        }
    }

    pub fn add_pot(&mut self, new_pot_amnt: i32) -> () {
        self.pot += new_pot_amnt
    }

    pub fn add_current_round_bet(&mut self, new_bet_amnt: i32) -> () {
        self.current_round_bet += new_bet_amnt
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

    pub fn flop(&mut self) -> () {
        if self.community_cards.len() != 0 {
            panic!("There are already community cards.");
        }

        let card1: Card = self.deck.remove_card();
        let card2: Card = self.deck.remove_card();
        let card3: Card = self.deck.remove_card();

        self.community_cards.push(card1);
        self.community_cards.push(card2);
        self.community_cards.push(card3);
    }

    pub fn turn(&mut self) -> () {
        if self.community_cards.len() != 3 {
            panic!("The flop hasn't happened.");
        }

        let card1: Card = self.deck.remove_card();

        self.community_cards.push(card1);
    }

    pub fn river(&mut self) -> () {
        if self.community_cards.len() != 4 {
            panic!("The turn hasn't happened.");
        }

        let card1: Card = self.deck.remove_card();

        self.community_cards.push(card1);
    }

    pub fn ask_user_action(&mut self) -> () {
        let mut action = String::new();
        let user_player: &mut Player = self.players.get_mut(0).unwrap();
        let current_round_bet = self.current_round_bet;
        let (first_card, second_card) = (
            user_player.hand.get(0).unwrap(),
            user_player.hand.get(1).unwrap(),
        );

        println!("Your stack: ${}", user_player.stack);
        println!("Current round bet: ${}", current_round_bet);
        println!("Current round bet: ${}", self.pot);
        println!("Your hand:");
        println!("Card 1: {:?} of {:?}", first_card.rank, first_card.suite);
        println!("Card 2: {:?} of {:?}", second_card.rank, second_card.suite);
        println!("Community Cards:");

        'get_action: loop {
            action.clear();

            println!("You can choose to fold, check, call, or raise.");
            io::stdin().read_line(&mut action).expect("Failed to read action input.");

            action = action.trim().to_lowercase();

            match action.as_str() {
                "fold" => {
                    user_player.fold();

                    break;
                },
                "check" => {
                    match user_player.check(current_round_bet) {
                        Ok(val) => self.add_pot(val),
                        Err(msg) => {
                            println!("{}", msg);
                            continue 'get_action;
                        },
                    }
                    break;
                },
                "call" => {
                    match user_player.call(current_round_bet) {
                        Ok(val) => self.add_pot(val),
                        Err(msg) => {
                            println!("{}", msg);
                            continue 'get_action;
                        },
                    }
                    break;
                },
                "raise" => {
                    let mut raise_amnt = String::new();
                    println!("How much do you want to raise?");
                    io::stdin().read_line(&mut raise_amnt).expect("Failed to read raise amount input.");
                    
                    let raise_amnt_i32 = raise_amnt.trim().parse::<i32>().unwrap();

                    match user_player.raise(raise_amnt_i32, current_round_bet) {
                        Ok(val) => self.add_pot(val),
                        Err(msg) => {
                            println!("{}", msg);
                            continue 'get_action;
                        },
                    }

                    break;
                },
                _ => {
                    println!("{} is not a valid action", action);

                    continue;
                },
            }
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