use rusty_poker::poker::{ PokerGame };

#[allow(unused_variables, unused_mut)]
fn main() {
    let mut game = PokerGame::new(4, 50);
    game.shuffle_deck();
    game.deal_cards();

    game.print_players();
    println!("-------------------");
    // game.print_cards();
    // println!("cards amnt: {}", game.deck.cards.len());

    let player1 = game.players.get_mut(0).unwrap();
    player1.raise(20);

    let player2 = game.players.get_mut(1).unwrap();
    player2.call(10);
    game.print_players();


    println!("Resetting deck.");
    game.reset_deck();
    println!("cards amnt: {}", game.deck.cards.len());

}
