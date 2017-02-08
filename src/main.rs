extern crate rand;

use rand::{thread_rng, Rng};

#[derive(Debug)]
enum SUIT {
    CLUB,
    DIAMOND,
    HEART,
    SPADE,
}



struct Card {
    suit: SUIT,
    rank: u8,
}

impl Card {
    fn to_string(&self) -> String {
        return format!("RANK: {}, SUIT: {:?}", self.rank, self.suit);
    }
}

// struct Table {
//     fountain_piles: Vec<FoundationPile>,
//     Piles: Vec<Pile>,
// }


struct Deck {
    cards: Vec<Card>

}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = vec![];

        for c in 1..14 {
            cards.push(Card {rank: c, suit: SUIT::CLUB});
        }

        for c in 1..14 {
            cards.push(Card {rank: c, suit: SUIT::DIAMOND});
        }

        for c in 1..14 {
            cards.push(Card {rank: c, suit: SUIT::HEART});
        }

        for c in 1..14 {
            cards.push(Card {rank: c, suit: SUIT::SPADE});
        }

        return Deck { cards: cards };
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
    }
}


fn main() {

    let mut deck = Deck::new();
    deck.shuffle();
    for c in deck.cards {
        println!("{}", c.to_string());
    }
}
