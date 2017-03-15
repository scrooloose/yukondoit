extern crate rand;
extern crate unicode_names;
extern crate colored;
#[macro_use]
extern crate itertools;
use std::io;
use std::io::Write;
use rand::{thread_rng, Rng};
use itertools::join;
use colored::*;

struct Suit {
    name: &'static str,
}

static SUITS: [Suit; 4] = [Suit { name: "CLUB" },
                           Suit { name: "DIAMOND" },
                           Suit { name: "HEART" },
                           Suit { name: "SPADE" }];

struct Rank {
    name: &'static str,
}

static RANKS: [Rank; 13] = [Rank { name: "ACE" },
                            Rank { name: "TWO" },
                            Rank { name: "THREE" },
                            Rank { name: "FOUR" },
                            Rank { name: "FIVE" },
                            Rank { name: "SIX" },
                            Rank { name: "SEVEN" },
                            Rank { name: "EIGHT" },
                            Rank { name: "NINE" },
                            Rank { name: "TEN" },
                            Rank { name: "JACK" },
                            Rank { name: "QUEEN" },
                            Rank { name: "KING" }];

struct Card {
    suit: &'static Suit,
    rank: &'static Rank,
}

impl Card {
    fn to_char(&self) -> Option<char> {
        let unicode_name = format!(
            "PLAYING CARD {} OF {}S",
            self.rank.name,
            self.suit.name,
        );
        return unicode_names::character(&unicode_name);
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        return Deck {
            cards: iproduct!(SUITS.iter(), RANKS.iter())
                .map(|(suit, rank)| {
                    Card {
                        suit: suit,
                        rank: rank,
                    }
                })
                .collect(),
        };
    }

    fn shuffle(mut self) -> Deck {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
        return self;
    }
}

struct Column<'a> {
    hidden_index: usize,
    cards: Vec<&'a Card>,
}

struct Table<'a> {
    columns: Vec<Column<'a>>,
}

impl<'a> Clone for Table<'a> {
    fn clone(&self) -> Self {
        return Table {
            columns: self.columns
                .iter()
                .map(|column| {
                    Column {
                        hidden_index: column.hidden_index,
                        cards: column.cards.clone(),
                    }
                })
                .collect(),
        };
    }
}

impl<'a> Table<'a> {
    fn move_card(&self, source: Coordinate, destination: Coordinate) -> Self {
        let mut new_table = self.clone();
        let moving_cards = new_table.columns[source.x].cards.split_off(source.y);
        new_table.columns[destination.x].cards.extend(moving_cards);
        return new_table;
    }
}

fn draw(table: &Table) {
    let mut column_card_iterators = table.columns
        .iter()
        .map(|column| column.cards.iter())
        .collect::<Vec<_>>();
    print!("\t{}\n\n",
           join((0..table.columns.len()).map(|i| i.to_string()), "\t"));
    let mut row_index = 0;
    loop {
        let mut row = vec![row_index.to_string().white()];
        let mut card_found = false;
        for (column_index, column) in table.columns.iter().enumerate() {
            let mut card_char = match column_card_iterators[column_index].next() {
                Some(card) => {
                    card_found = true;
                    let card_string = card.to_char().unwrap().to_string();
                    match card.suit.name {
                        "DIAMOND" | "HEART" => card_string.red(),
                        _ => card_string.white(),
                    }
                }
                None => "-".to_string().white(),
            };
            if card_found && row_index < column.hidden_index {
                card_char = "X".to_string().white();
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

fn deal(deck: &Deck) -> Table {
    let mut columns = vec![];
    let mut start = 0;
    let mut end = 1;
    for column_number in 0..7 {
        columns.push(Column {
            hidden_index: column_number,
            cards: deck.cards[start..end].iter().collect(),
        });
        start = end;
        end = end + 6 + column_number;
    }
    return Table { columns: columns };
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn read_coordinate(message: &str) -> Coordinate {
    println!("{}", message);
    io::stdout().write(b"x: ").ok();
    io::stdout().flush().ok();
    let mut x = String::new();
    io::stdin().read_line(&mut x).ok();
    let x: usize = x.trim().parse().expect("x must be an int");

    io::stdout().write(b"y: ").ok();
    io::stdout().flush().ok();
    let mut y = String::new();
    io::stdin().read_line(&mut y).ok();
    let y: usize = y.trim().parse().expect("y must be an int");

    return Coordinate { x: x, y: y };
}

fn main() {
    let deck = Deck::new().shuffle();
    (0..).fold(deal(&deck), |t, _| {
        draw(&t);
        let source = read_coordinate("Enter a source coordinate.");
        let destination = read_coordinate("Enter a destination coordinate.");
        return t.move_card(source, destination);
    });
}
