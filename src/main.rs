use std::io;
use std::fmt;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
enum RankValue {
    Standard(u8),
    Ace(u8, u8)
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
struct Card {
    rank: CardRank,
    suit: CardSuit
}

impl Card {
    fn value(&self) -> RankValue {
        self.rank.value()
    }

    fn display(&self) -> String {
        ["[", self.rank.display(), self.suit.display(), "]"].join(" ")
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}", self.display())
    }
}

#[derive(Debug)]
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

    fn display(&self) -> String {
        self.0.iter().map(|card| card.display()).collect::<Vec<String>>().join(" ")
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
        let mut rng = rand::thread_rng();
        let deck_slice = self.0.as_mut_slice();

        deck_slice.shuffle(&mut rng);
    }

    fn new() -> Deck {
        let mut deck = Deck(Self::build_ordered());
        deck.shuffle();

        deck
    }

    fn take(&mut self) -> Card {
        self.0.pop().unwrap()
    }

    fn deal_cards(&mut self) -> Vec<Card> {
        vec![ self.take(), self.take() ]
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    hand: Hand,
    is_computer: bool,
    is_playing: bool,

}

impl Player {
    fn new(name: String, is_computer: bool) -> Player {
        Player { name: name, hand: Hand(Vec::new()), is_computer: is_computer, is_playing: true }
    }

    fn take_computer_turn(&mut self) {
        let self_limit: f32 = (rand::thread_rng().gen::<f32>() * 4.0).ceil() + 15.0;

        if f32::from(self.hand.score()) >= self_limit {
            println!("{} decided to stand.", self.name);
            self.is_playing = false;
        } else {
            println!("{} decided to hit.", self.name);
        }
    }

    fn take_user_turn(&mut self) {
        println!("Would you like to [hit] or [stand]?");

        let mut player_decision = String::new();

        io::stdin().read_line(&mut player_decision)
            .expect("Failed to read the line!"); // TODO: actually handle the error

        let player_decision: &str = player_decision.trim();

        if player_decision == "stand" {
            self.is_playing = false;
        } else if player_decision != "hit" {
            println!("Unknown move. Please enter \"hit\" or \"stand\".");
            self.take_user_turn();
        }
    }

    fn take_turn(&mut self, deck: &mut Deck) {
        if self.is_computer {
            self.take_computer_turn();
        }

        else {
            self.take_user_turn();
        }

        if self.is_playing {
            self.hand.0.push(deck.take());
        }

        println!("{}", self);

        // end play for a busted hand
        if self.hand.score() > 21 {
            println!("This hand has busted. Better luck next time.");
            self.is_playing = false;
        }

        println!("");
    }

    fn resolve_dealer(&mut self, deck: &mut Deck) {
        println!("Resolving the dealer's hand...");
        while self.hand.score() < 17 {
            self.hand.0.push(deck.take());
            println!("{}", self);
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}'s hand: {}  |  score: {}",
               self.name, self.hand.display(), self.hand.score())
    }
}

struct Game {
    dealer: Player,
    players: Vec<Player>,
    deck: Deck
}

impl Game {
    fn get_player_count() -> u8 {
        println!("How many players (including yourself)?");

        let mut player_count = String::new();

        io::stdin().read_line(&mut player_count)
            .expect("Failed to read the line!"); // TODO: actually handle the error

        let player_count: u8 = player_count.trim().parse()
            .expect("Please enter a number between 1 and 9."); // TODO: actually handle the error

        player_count
    }

    fn deal_in(&mut self) {
        for player in &mut self.players {
            player.hand.0.append(&mut self.deck.deal_cards());
        }

        self.dealer.hand.0.push(self.deck.take());
    }

    fn print(&self) {
        // show what each player has
        for player in &self.players {
            println!("{}", player);
        }

        // show what the dealer has
        println!("{}", self.dealer);
    }

    fn find_winner(&self) -> Option<&Player> {
        let closest_player = self.players.iter()
            .filter(|p| p.hand.score() <= 21) // unbusted
            .max_by(|p_1, p_2| p_1.hand.score().cmp(&p_2.hand.score()));

        // TODO: memoize score on the Player struct so we don't have to recalc for each of these
        match closest_player {
            Some(cp) => {
                // if dealer tops, dealer wins.
                if self.dealer.hand.score() <= 21 && self.dealer.hand.score() > cp.hand.score() {
                    return Some(&self.dealer);
                }

                // if not, make sure we don't have ties. if ties, nobody wins.
                let top_players = self.players.iter()
                    .filter(|p| p.hand.score() == cp.hand.score());

                if top_players.count() > 1 || cp.hand.score() == self.dealer.hand.score() {
                    return None;
                }

                return Some(cp);
            },

            None => {
                // if every player busted, see if dealer busted or not
                if self.dealer.hand.score() <= 21 {
                    return Some(&self.dealer);
                }

                return None;
            }
        }
    }

    fn play(&mut self) -> bool {
        self.print();

        for player in &mut self.players {
            if player.is_playing {
                player.take_turn(&mut self.deck);
            }
        }

        let has_playing_players = self.players.iter().any(|e| e.is_playing);

        println!("");

        if !has_playing_players {
            self.dealer.resolve_dealer(&mut self.deck);

            let winner = self.find_winner();

            match winner {
                Some(w) => println!("{} has won the game!", w.name),
                None => println!("Nobody won this game.")
            };

            return false;
        }

        return true;
    }

    fn new() -> Game {
        let player_count = Self::get_player_count();
        let deck = Deck::new();
        let mut players = Vec::new();

        players.push(Player::new(String::from("You"), false));

        for p in 2..player_count+1 {
            players.push(Player::new(["Player ", &p.to_string()].join(" "), true));
        }

        Game {
            deck: deck,
            dealer: Player::new(String::from("Mr. Zamboni"), true),
            players: players
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.deal_in();

    loop {
        println!("");

        let should_continue = game.play();

        if !should_continue {
            break;
        }
    }
}
