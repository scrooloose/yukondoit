extern crate rand;
extern crate unicode_names;
#[macro_use] extern crate itertools;
use rand::{thread_rng, Rng};
#[derive(Debug, Clone, Copy)]
struct Card {
    suit: usize,
    rank: usize,
}


fn shuffle<T>(original_vector: Vec<T>) -> Vec<T> {
    let mut shuffled_vector = original_vector;
    let mut rng = thread_rng();
    rng.shuffle(&mut shuffled_vector);
    return shuffled_vector;
}

fn suit_name_by_index(suit_index: usize) -> Option<&'static str> {
    return match suit_index {
        0 => Some("CLUB"),
        1 => Some("DIAMOND"),
        2 => Some("HEART"),
        3 => Some("SPADE"),
        _ => None,
    }
}

fn rank_name_by_index(rank_index: usize) -> Option<&'static str> {
    return match rank_index {
        1 => Some("ACE"),
        2 => Some("TWO"),
        3 => Some("THREE"),
        4 => Some("FOUR"),
        5 => Some("FIVE"),
        6 => Some("SIX"),
        7 => Some("SEVEN"),
        8 => Some("EIGHT"),
        9 => Some("NINE"),
        10 => Some("TEN"),
        11 => Some("JACK"),
        12 => Some("QUEEN"),
        13 => Some("KING"),
        _ => None,
    }
}

fn char_for_card(card: Card) -> Option<char>{
    let unicode_name = format!(
        "PLAYING CARD {} OF {}S",
        rank_name_by_index(card.rank).unwrap(),
        suit_name_by_index(card.suit).unwrap(),
    );
    return unicode_names::character(&unicode_name);
}

fn new_deck() -> Vec<Card>{
    let suits = 0..4;
    let ranks = 1..14;
    return iproduct!(suits, ranks).map(
        |(suit_index, rank_index)| {
            Card {
                suit: suit_index,
                rank: rank_index,
            }
        }
    ).collect::<Vec<_>>();
}

fn draw(deck: Vec<Card>) {
    let mut cards = deck.iter();
    let mut row = 0;
    for i in (0..7).cycle() {
        match cards.next() {
            Some(card) => {
                if i == 0 {
                    print!("{}\t", row);
                }
                print!("{}", char_for_card(*card).unwrap());
                if i == 6 {
                    row = row + 1;
                    print!("\n\n");
                } else {
                    print!("\t");
                }
            },
            None => {
                break;
            }
        }
    }
    print!("\n");
}

fn main() {
    let deck = shuffle(new_deck());

    draw(deck);
}
