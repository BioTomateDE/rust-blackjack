use std::fmt::{Display, Formatter};
use colored::Colorize;
use rand::prelude::SliceRandom;
use rand::rng;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

impl Display for CardNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
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
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum CardColor {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

#[derive(Clone)]
pub struct Card {
    number: CardNumber,
    color: CardColor,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.color {
            CardColor::Diamonds => write!(f, "{}", format!("♦ {}", self.number).red()),
            CardColor::Hearts => write!(f, "{}", format!("♥ {}", self.number).red()),
            CardColor::Spades => write!(f, "{}", format!("♠ {}", self.number)),
            CardColor::Clubs => write!(f, "{}", format!("♣ {}", self.number)),
        }
    }
}


fn get_card_number_value(card_number: CardNumber) -> u8 {
    match card_number {
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

pub fn get_card_sum(cards: &Vec<Card>) -> u8 {
    let mut sum: u8 = 0;
    for card in cards {
        if card.number == CardNumber::Ace && sum >= 11 {
            sum += 1;
        } else {
            sum += get_card_number_value(card.number);
        }
    }
    sum
}


fn gen_cards_for_color(card_color: CardColor) -> Vec<Card> {
    vec![
        Card{ number: CardNumber::Two, color: card_color },
        Card{ number: CardNumber::Three, color: card_color },
        Card{ number: CardNumber::Four, color: card_color },
        Card{ number: CardNumber::Five, color: card_color },
        Card{ number: CardNumber::Six, color: card_color },
        Card{ number: CardNumber::Seven, color: card_color },
        Card{ number: CardNumber::Eight, color: card_color },
        Card{ number: CardNumber::Nine, color: card_color },
        Card{ number: CardNumber::Ten, color: card_color },
        Card{ number: CardNumber::Jack, color: card_color },
        Card{ number: CardNumber::Queen, color: card_color },
        Card{ number: CardNumber::King, color: card_color },
        Card{ number: CardNumber::Ace, color: card_color },
    ]
}

pub fn gen_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    deck.extend(gen_cards_for_color(CardColor::Diamonds));
    deck.extend(gen_cards_for_color(CardColor::Hearts));
    deck.extend(gen_cards_for_color(CardColor::Spades));
    deck.extend(gen_cards_for_color(CardColor::Clubs));
    deck.shuffle(&mut rng());
    deck
}


pub fn fmt_cards(cards: &Vec<Card>) -> String {
    let mut buf: String = String::new();
    buf.push('[');
    
    for card in cards {
        buf.push_str(&format!("{card}, "));
    }
    
    buf.pop();
    buf.pop();
    buf.push(']');
    buf
}

