// use std::io;
use std::fmt;
use rand::seq::SliceRandom;
use rand::thread_rng;

enum RankValue {
    Standard(u8),
    Ace(u8, u8)
}

#[derive(Clone)]
#[derive(Copy)]
enum CardRank {
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

impl CardRank {
    fn display(&self) -> &str {
        match self {
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
        }
    }

    fn value(&self) -> RankValue {
        match self {
            Self::Ace => RankValue::Ace(1, 11),
            Self::Two => RankValue::Standard(2),
            Self::Three => RankValue::Standard(3),
            Self::Four => RankValue::Standard(4),
            Self::Five => RankValue::Standard(5),
            Self::Six => RankValue::Standard(6),
            Self::Seven => RankValue::Standard(7),
            Self::Eight => RankValue::Standard(8),
            Self::Nine => RankValue::Standard(9),
            Self::Ten | Self:: Jack | Self::Queen | Self::King => RankValue::Standard(10)
        }
    }
}

#[derive(Clone)]
#[derive(Copy)]
enum CardSuit {
    Club,
    Diamond,
    Heart,
    Spade
}

impl CardSuit {
    fn display(&self) -> &str {
        match self {
            Self::Club => "\u{2663}",
            Self::Diamond => "\u{2666}",
            Self::Heart => "\u{2665}",
            Self::Spade => "\u{2660}"
        }
    }
}

#[derive(Clone)]
#[derive(Copy)]
struct Card {
    rank: CardRank,
    suit: CardSuit
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "[{} {}]", self.rank.display(), self.suit.display())
    }
}

impl Card {
    fn value(&self) -> RankValue {
        self.rank.value()
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
                RankValue::Standard(val) => score += val,
                RankValue::Ace(low, high) => aces.push((low, high))
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

struct Deck(Vec<Card>);

impl Deck {
    fn build_ordered() -> Vec<Card> {
        let rank_options = [
            CardRank::Ace,
            CardRank::Two,
            CardRank::Three,
            CardRank::Four,
            CardRank::Five,
            CardRank::Six,
            CardRank::Seven,
            CardRank::Eight,
            CardRank::Nine,
            CardRank::Ten,
            CardRank::Jack,
            CardRank::Queen,
            CardRank::King
        ];

        rank_options.iter()
            .map(|opt| [
                 Card { rank: opt.clone(), suit: CardSuit::Diamond },
                 Card { rank: opt.clone(), suit: CardSuit::Club },
                 Card { rank: opt.clone(), suit: CardSuit::Heart },
                 Card { rank: opt.clone(), suit: CardSuit::Spade }
            ])
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
    let mut deck = Deck(Vec::new());
    deck.build();

    for card in deck.0 {
        print!("{} ", card);
    }
}
