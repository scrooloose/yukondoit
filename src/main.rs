extern crate rand;
extern crate unicode_names;
#[macro_use] extern crate itertools;

use rand::{thread_rng, Rng};
use itertools::join;

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

fn char_for_card(card: Card) -> Option<char> {
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

struct Column {
    hidden_index: usize,
    cards: Vec<Card>,
}

struct Table {
    columns: Vec<Column>,
}

fn deal(deck: Vec<Card>) -> Table {
    let mut columns = vec![];
    let mut start = 0;
    let mut end = 1;
    for column_number in 0..7 {
        columns.push(
            Column {
                hidden_index: column_number,
                cards: deck[start..end].to_vec(),
            }
        );
        start = end;
        end = end + 6 + column_number;
    }
    return Table {
        columns: columns,
    }
}

fn draw(table: Table) {
    let mut column_card_iterators = table.columns.iter().map(
        |column| {
            column.cards.iter()
        }
    ).collect::<Vec<_>>();
    print!(
        "\t{}\n\n",
        join(
            (1..1 + table.columns.len()).map(|i| i.to_string()),
            "\t"
        )
    );
    let mut row_index = 0;
    loop {
        let mut row = vec![row_index.to_string()];
        let mut card_found = false;
        for (column_index, column) in table.columns.iter().enumerate() {
            let mut card_char = match column_card_iterators[column_index].next() {
                Some(card) => {
                    card_found = true;
                    char_for_card(*card).unwrap().to_string()
                },
                None => "-".to_string(),
            };
            if card_found && row_index < column.hidden_index {
                card_char = "X".to_string();
            };
            row.push(card_char);
        }
        if card_found {
            print!("{}\n\n", join(row, "\t"));
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
    let table = deal(shuffled_deck);
    draw(table);
}
