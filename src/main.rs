extern crate rand;
extern crate unicode_names;
#[macro_use] extern crate itertools;

#[derive(Debug, Clone, Copy)]
struct Card {
    suit: usize,
    rank: usize,
}


// fn shuffle(&mut self) {
//     let mut rng = thread_rng();
//     rng.shuffle(&mut self.cards);
// }

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

fn main() {
    let mut row = 0;
    let suits = 0..4;
    let ranks = 1..14;
    let mut cards = iproduct!(suits, ranks).map(
        |(suit_index, rank_index)| {
            Card {
                suit: suit_index,
                rank: rank_index,
            }
        }
    );

    for i in (0..7).cycle() {
        match cards.next() {
            Some(card) => {
                if i == 0 {
                    print!("{}\t", row);
                }
                print!("{}", char_for_card(card).unwrap());
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
