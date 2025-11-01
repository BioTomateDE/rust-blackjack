use crate::deck::DECK;
use colored::{ColoredString, Colorize};
use rand::prelude::SliceRandom;
use rand::rng;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardNumber {
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
    King,
    Ace,
}

impl CardNumber {
    pub const fn value(&self) -> u8 {
        match self {
            CardNumber::Two => 2,
            CardNumber::Three => 3,
            CardNumber::Four => 4,
            CardNumber::Five => 5,
            CardNumber::Six => 6,
            CardNumber::Seven => 7,
            CardNumber::Eight => 8,
            CardNumber::Nine => 9,
            CardNumber::Ten => 10,
            CardNumber::Jack => 10,
            CardNumber::Queen => 10,
            CardNumber::King => 10,
            CardNumber::Ace => 11,
        }
    }
}

impl Display for CardNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: &str = match self {
            CardNumber::Two => "2",
            CardNumber::Three => "3",
            CardNumber::Four => "4",
            CardNumber::Five => "5",
            CardNumber::Six => "6",
            CardNumber::Seven => "7",
            CardNumber::Eight => "8",
            CardNumber::Nine => "9",
            CardNumber::Ten => "10",
            CardNumber::Jack => "Jack",
            CardNumber::Queen => "Queen",
            CardNumber::King => "King",
            CardNumber::Ace => "Ace",
        };
        write!(f, "{string}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardColor {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    number: CardNumber,
    color: CardColor,
}

impl Card {
    pub const fn new(number: CardNumber, color: CardColor) -> Self {
        Self { number, color }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n: CardNumber = self.number;
        let string: ColoredString = match self.color {
            CardColor::Diamonds => format!("♦ {n}").bright_red().bold(),
            CardColor::Hearts => format!("♥ {n}").bright_red().bold(),
            CardColor::Spades => format!("♠ {n}").bright_white().bold(),
            CardColor::Clubs => format!("♣ {n}").bright_white().bold(),
        };
        write!(f, "{string}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck(Vec<Card>);
impl Deck {
    /// Generates a new 52 card deck and shuffles it.
    pub fn new() -> Self {
        let mut deck: Vec<Card> = DECK.to_vec();
        deck.shuffle(&mut rng());
        Self(deck)
    }

    /// Pops the last card off the deck and returns it.
    /// This function will panic if the deck is empty.
    pub fn pop_card(&mut self) -> Card {
        self.0.pop().expect("Deck is empty")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A hand of cards (either player or dealer).
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(card1: Card, card2: Card) -> Self {
        Self(vec![card1, card2])
    }

    /// Get the first card of the hand ("upcard").
    /// This function will panic if the hand is empty (although this should never happen).
    pub fn upcard(&self) -> &Card {
        self.0.first().expect("Hand is empty")
    }

    /// How many cards this hand currently holds.
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Add a card to this hand.
    pub fn push_card(&mut self, card: Card) {
        self.0.push(card);
    }

    /// Get the sum of the card values, accounting for "soft cards" (regarding aces).
    pub fn sum(&self) -> u8 {
        let mut sum: u8 = 0;
        for card in &self.0 {
            if card.number == CardNumber::Ace && sum >= 11 {
                sum += 1;
            } else {
                sum += card.number.value();
            }
        }
        sum
    }

    /// Prints out `Your Sum: {} | Your Cards: {}` for `who = "Your"`.
    pub fn print_info(&self, who: &str) {
        println!("{who} Sum: {} | {who} Cards: {}", self.sum(), self);
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        for (i, card) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", card)?;
        }

        write!(f, "]")
    }
}
