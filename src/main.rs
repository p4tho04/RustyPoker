use rusty_poker::poker::{ PokerGame };

#[allow(unused_variables, unused_mut)]
fn main() {
    let mut game = PokerGame::new(4, 70);
    game.shuffle_deck();
    game.deal_cards();

    game.print_players();
    println!("-------------------");

    game.flop();
    game.turn();
    game.river();

    game.add_current_round_bet(50);

    game.ask_user_action();

    game.print_players();
}
