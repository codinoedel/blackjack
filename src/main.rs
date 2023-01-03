// use std::io;
use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ace => "A",
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K"
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
enum CardValue {
    Standard(u8),
    Ace(u8, u8)
}

impl Card {
    fn value(&self) -> CardValue {
        match self {
            Self::Ace => CardValue::Ace(1, 11),
            Self::Two => CardValue::Standard(2),
            Self::Three => CardValue::Standard(3),
            Self::Four => CardValue::Standard(4),
            Self::Five => CardValue::Standard(5),
            Self::Six => CardValue::Standard(6),
            Self::Seven => CardValue::Standard(7),
            Self::Eight => CardValue::Standard(8),
            Self::Nine => CardValue::Standard(9),
            Self::Ten | Self:: Jack | Self::Queen | Self::King => CardValue::Standard(10)

        }
    }
}

struct Hand(Vec<Card>);

impl Hand {
    fn score(&self) -> u8 {
        let values = self.0.iter().map(|card| card.value());
        let mut score = 0;
        let mut aces = Vec::new();

        // count up standard values and number of aces
        for value in values {
            match value {
                CardValue::Standard(val) => score += val,
                CardValue::Ace(low, high) => aces.push((low, high))
            }
        }

        // find closest score to 21, ideally without going over
        for ace in aces {
            if score + ace.1 <= 21 {
                score += ace.1
            } else {
                score += ace.0
            }
        }

        score
    }
}

#[derive(Debug)]
struct Deck(Vec<Card>);

impl Deck {
    fn build_ordered() -> Vec<Card> {
        let options = [
            Card::Ace,
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Jack,
            Card::Queen,
            Card::King
        ];

        options.iter()
            .map(|opt| [opt.clone(), opt.clone(), opt.clone(), opt.clone()])
            .flatten()
            .collect()
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let deck_slice = self.0.as_mut_slice();

        deck_slice.shuffle(&mut rng);
    }

    fn build(&mut self) {
        self.0 = Deck::build_ordered();
        self.shuffle();
    }
}

// struct Player {
//     name: String,
//     hand: Hand
// }
//
// struct Game {
//     players: Vec<Player>,
//     deck: Vec<Card>,
//     table: Vec<Card>
// }

// fn get_player_count() -> u8 {
//     println!("How many players (including yourself)?");
//
//     let mut player_count = String::new();
//
//     io::stdin().read_line(&mut player_count)
//         .expect("Failed to read the line!"); // TODO: actually handle the error
//
//     let player_count: u8 = player_count.trim().parse()
//         .expect("Please enter a number between 5 and 9."); // TODO: actually handle the error
//
//     println!("Setting up game for {player_count} players...");
//
//     player_count
// }

// fn initialize_deck() -> Vec<Card> {
//     let mut deck: Vec<Card> = Vec::new();
//
//     for opt in options {
//         for _num in 1..4 {
//             deck.push(opt);
//         }
//     }
//
//     deck
// }
//
// fn make_game() {
//     let deck = initialize_deck();
// }
//
fn main() {
    println!("{}", Card::Ace);
    println!("{:?}", Card::Five.value());
    let mut deck = Deck(Vec::new());
    deck.build();
    println!("{:?}", deck.0);
}
