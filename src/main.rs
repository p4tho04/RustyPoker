use rusty_poker::deck::{ Deck };

fn main() {
    let deck = Deck::new();

    for card in deck.cards {
        println!("{:?}", card);
    }
}
