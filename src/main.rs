use rusty_poker::deck::{ Deck };

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    for card in deck.cards {
        println!("{:?}", card);
    }
}
