use rusty_poker::poker::{ PokerGame };

#[allow(unused_variables, unused_mut)]
fn main() {
    let mut game = PokerGame::new(4, 50);
    game.shuffle_deck();
    game.deal_cards();

    for card in game.deck.cards.iter() {
        println!("{:?}", card);
    }

    for player in game.players {
        println!("Player {}, {:?}", player.player_num, player);
    }
}
