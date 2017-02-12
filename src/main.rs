extern crate rand;
use self::SUIT::*;
use std::slice::Iter;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Copy)]
enum SUIT {
    CLUB,
    DIAMOND,
    HEART,
    SPADE,
}

impl SUIT {
    pub fn iterator() -> Iter<'static, SUIT> {
        static SUITS: [SUIT;  4] = [CLUB, DIAMOND, HEART, SPADE];
        return SUITS.into_iter();
    }
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
        for suit in SUIT::iterator() {
            for c in 1..14 {
                cards.push(Card {rank: c, suit: *suit});
            }
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
