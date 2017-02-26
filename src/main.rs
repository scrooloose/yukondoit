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

    pub fn char_for(value: SUIT) -> &'static str {
        return match value {
            CLUB => "C",
            DIAMOND => "D",
            HEART => "H",
            SPADE => "S"
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Card {
    suit: SUIT,
    rank: u8,
}

impl Card {
    fn to_string(&self) -> String {
        return format!("{}{}", SUIT::char_for(self.suit), self.rank);
    }
}


struct Pile {
    hidden_index: usize,
    cards: Vec<Card>,
}


struct Table {
    piles: Vec<Pile>,
}

impl Table {
    fn draw(&self) {
        let mut row = 0;
        loop {
            let mut any_match = false;
            for pile in self.piles.iter() {
                match pile.cards.get(row) {
                    Some(card) => {
                        if (row < pile.hidden_index) {
                            print!("X");
                        } else {
                            print!("{}", card.to_string());
                        }
                        any_match = true;
                    },
                    None => print!("."),
                }
                print!("\t")
            }
            print!("\n");
            if ! any_match {
                break;
            }
            row = row + 1;
        }
    }
}


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

    //SUIT::char_for(DIAMOND);
    let mut deck = Deck::new();
    deck.shuffle();
    let pile1 = Pile {cards: deck.cards[0..1].to_vec(), hidden_index: 0};
    let pile2 = Pile {cards: deck.cards[1..7].to_vec(), hidden_index: 1};
    let pile3 = Pile {cards: deck.cards[7..14].to_vec(), hidden_index: 2};
    let pile4 = Pile {cards: deck.cards[14..22].to_vec(), hidden_index: 3};
    let pile5 = Pile {cards: deck.cards[22..31].to_vec(), hidden_index: 4};
    let pile6 = Pile {cards: deck.cards[31..41].to_vec(), hidden_index: 5};
    let pile7 = Pile {cards: deck.cards[41..52].to_vec(), hidden_index: 6};
    let piles = vec![
        pile1, pile2, pile3, pile4, pile5, pile6, pile7,
    ];
    let t = Table{piles: piles};
    t.draw();
}
