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

#[derive(Debug)]
struct Suit {
    name: &'static str,
    color: &'static str,
}

impl PartialEq for Suit {
    fn eq(&self, other: &Suit) -> bool {
        self.name == other.name && self.color == other.color
    }
}

static SUITS: [Suit; 4] = [Suit {
                               name: "CLUB",
                               color: "BLACK",
                           },
                           Suit {
                               name: "DIAMOND",
                               color: "RED",
                           },
                           Suit {
                               name: "HEART",
                               color: "RED",
                           },
                           Suit {
                               name: "SPADE",
                               color: "BLACK",
                           }];

#[derive(Debug)]
struct Rank {
    name: &'static str,
    value: u8,
}

impl PartialEq for Rank {
    fn eq(&self, other: &Rank) -> bool {
        self.name == other.name && self.value == other.value
    }
}

static RANKS: [Rank; 13] = [Rank {
                                name: "ACE",
                                value: 1,
                            },
                            Rank {
                                name: "TWO",
                                value: 2,
                            },
                            Rank {
                                name: "THREE",
                                value: 3,
                            },
                            Rank {
                                name: "FOUR",
                                value: 4,
                            },
                            Rank {
                                name: "FIVE",
                                value: 5,
                            },
                            Rank {
                                name: "SIX",
                                value: 6,
                            },
                            Rank {
                                name: "SEVEN",
                                value: 7,
                            },
                            Rank {
                                name: "EIGHT",
                                value: 8,
                            },
                            Rank {
                                name: "NINE",
                                value: 9,
                            },
                            Rank {
                                name: "TEN",
                                value: 10,
                            },
                            Rank {
                                name: "JACK",
                                value: 11,
                            },
                            Rank {
                                name: "QUEEN",
                                value: 12,
                            },
                            Rank {
                                name: "KING",
                                value: 13,
                            }];

#[derive(Debug)]
struct Card {
    suit: &'static Suit,
    rank: &'static Rank,
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}

impl Card {
    fn to_char(&self) -> char {
        let unicode_name = format!(
            "PLAYING CARD {} OF {}S",
            self.rank.name,
            self.suit.name,
        );
        return unicode_names::character(&unicode_name).unwrap()
    }
    fn to_string(&self) -> ColoredString {
        let character = self.to_char().to_string();
        if self.suit.color == "RED" {
            return character.red();
        } else {
            return character.white();
        }
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

    fn card_from_char(&self, card_char: char) -> Option<&Card> {
        for card in self.cards.iter() {
            if card.to_char() == card_char {
                return Some(card);
            }
        }
        return None;
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
    fn move_card(&self, next_move: Move) -> Self {
        let mut new_table = self.clone();
        for column in new_table.columns.iter_mut() {
            for (i, &card) in column.cards.iter().enumerate() {
                if card == next_move.source {
                    let moving_cards = column.cards.split_off(i);

                    break;
                }
            }
        }
        //let moving_cards = new_table.columns[source.x].cards.split_off(source.y);
        // new_table.columns[destination.x].cards.extend(moving_cards);
        return new_table;
    }
}

fn draw(table: &Table) {
    let movable_cards = movable_cards(&table);
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
                    let mut card_string = card.to_string();
                    if movable_cards.contains(&card) {
                        card_string = card_string.bold();
                    } else {
                        card_string = card_string.dimmed();
                    }
                    card_string
                }
                None => "-".to_string().white(),
            };
            if card_found {
                if row_index < column.hidden_index {
                    card_char = "x".to_string().white();
                }
            }
            row.push(card_char);
        }
        if card_found {
            print!("{}\n\n", join(row, "\t"));
        } else {
            break;
        }
        row_index += 1;
    }
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

fn read_card(prompt: &[u8]) -> char {
    io::stdout().write(prompt).ok();
    io::stdout().flush().ok();
    let mut card_char = String::new();
    io::stdin().read_line(&mut card_char).ok();
    return card_char.chars().nth(0).unwrap();
}

#[derive(Debug)]
struct Move<'a> {
    source: &'a Card,
    destination: &'a Card,
}

fn next_cards<'a, 'b>(table: &'a Table, last_card: &'b Card) -> Vec<&'a Card> {
    let mut next = vec![];
    for column in table.columns.iter() {
        if column.cards.contains(&last_card) {
            continue;
        }
        for card in column.cards.iter().skip(column.hidden_index) {
            if card.rank.value != last_card.rank.value - 1 {
                continue;
            }
            if card.suit.color == last_card.suit.color {
                continue;
            }
            next.push(*card);
        }
    }
    return next;
}

fn movable_cards<'a>(table: &'a Table) -> Vec<&'a Card>{
    let mut movable_cards = vec![];
    for column in table.columns.iter() {
        let last_card = column.cards.last().unwrap();
        for card in next_cards(&table, last_card) {
            movable_cards.push(card);
        }
    }
    return movable_cards;
}


fn main() {
    let deck = Deck::new().shuffle();
    (0..).fold(deal(&deck), |t, _| {
        draw(&t);
        let source_char = read_card(b"Source: ");
        println!("{:?}", source_char);
        let destination_char = read_card(b"Destination: ");
        println!("{:?}", destination_char);
        let source_card = &deck.card_from_char(source_char).unwrap();
        println!("{:?}", source_card);
        let destination_card = &deck.card_from_char(destination_char).unwrap();
        println!("{:?}", destination_card);
        let next_move = Move {
            source: source_card,
            destination: destination_card,
        };
        println!("{:?}", next_move);
        return t.move_card(next_move);
    });
}
