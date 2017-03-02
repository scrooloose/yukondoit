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

fn new_deck() -> Vec<Card> {
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

fn deal(deck: Vec<Card>) -> Vec<Vec<Card>> {
    let mut columns = vec![];
    for _ in 0..7 {
        columns.push(vec![]);
    }
    for (card, column_index) in izip!(deck, (0..7).cycle()) {
        columns[column_index].push(card);
    }
    return columns;
}

fn draw(columns: Vec<Vec<Card>>) {
    let mut column_iterators = vec![];
    for column in columns.iter() {
        column_iterators.push(column.iter());
    }
    let mut row_index = 0;
    loop {
        let mut card_found = false;
        let mut row = format!("{}\t", row_index);
        for column_iterator in column_iterators.iter_mut() {
            match column_iterator.next() {
                Some(card) => {
                    card_found = true;
                    row = format!("{}{}", row, char_for_card(*card).unwrap());
                },
                None => {
                    row = format!("{}-", row);
                },
            };
            row += "\t";
        }
        if card_found {
            print!("{}\n\n", row);
        } else {
            break;
        }
        row_index += 1;
    }
    print!("\n");
}

fn main() {
    let deck = new_deck();
    let shuffled_deck = shuffle(deck);
    let columns = deal(shuffled_deck);
    draw(columns);
}
