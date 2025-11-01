mod card;
mod deck;
mod input;

use crate::card::{Deck, Hand};
use crate::input::{get_action_input, get_bet};
use colored::Colorize;
use rand::prelude::IndexedRandom;
use rand::rng;
use std::cmp::Ordering;
use std::io::BufRead;
use std::thread::sleep;
use std::time::Duration;

enum Action {
    Stand,
    Hit,
    Double,
}

fn sleep_ms(ms: u64) {
    if std::env::args().nth(1).as_deref() == Some("nosleep") {
        return;
    }
    let sleep_mode = std::env::var("BJ_SLEEP").unwrap_or_default();
    match sleep_mode.as_str() {
        "disabled" | "0" | "false" => return,
        _ => {}
    }
    sleep(Duration::from_millis(ms));
}

/// Returns the money gained (can be negative if lost).
fn play(mut bet: u64) -> i64 {
    let mut deck = Deck::new();
    let mut dealer_cards = Hand::new(deck.pop_card(), deck.pop_card());
    let mut player_cards = Hand::new(deck.pop_card(), deck.pop_card());

    sleep_ms(230);
    println!("Dealer Upcard: {}", dealer_cards.upcard());
    player_cards.print_info("Your");
    println!();

    // Player Blackjack; 3/2 payout.
    if player_cards.sum() == 21 {
        let payout: i64 = bet as i64 * 3 / 2;
        println!("You won {payout}$ by getting a blackjack!");
        return payout;
    }

    // Dealer Blackjack.
    if dealer_cards.sum() == 21 {
        dealer_cards.print_info("Dealer");
        println!("You lost {bet}$ because the dealer has a blackjack!");
        return -(bet as i64);
    }

    // Player action input until stand, double, 21 or busted.
    while player_cards.sum() < 21 {
        let double_allowed: bool = player_cards.count() == 2;
        let action: Action = get_action_input(double_allowed);

        match action {
            Action::Stand => {
                println!("You stood on {}.", player_cards.sum());
                break;
            }
            Action::Hit => player_cards.push_card(deck.pop_card()),
            Action::Double => {
                player_cards.push_card(deck.pop_card());
                player_cards.print_info("Your");
                bet *= 2;
                break;
            }
        }
        sleep_ms(187);
        player_cards.print_info("Your");
    }

    // Player busted.
    let sum: u8 = player_cards.sum();
    if sum > 21 {
        let message = format!("You busted with a sum of {sum} and lost {bet}$!");
        println!("{}", message.red());
        return -(bet as i64);
    }

    sleep_ms(320);
    dealer_cards.print_info("Dealer");

    // Dealer draws cards until they have a sum of 17 or higher.
    while dealer_cards.sum() < 17 {
        sleep_ms(800);
        dealer_cards.push_card(deck.pop_card());
        dealer_cards.print_info("Dealer");
    }

    // Dealer busted.
    let sum: u8 = dealer_cards.sum();
    if sum > 21 {
        let msg = format!("You won {bet}$ because the dealer busted with a sum of {sum}!");
        println!("{}", msg.bright_green());
        return bet as i64;
    }
    sleep_ms(530);

    let player_sum: u8 = player_cards.sum();
    let dealer_sum: u8 = dealer_cards.sum();

    match player_sum.cmp(&dealer_sum) {
        Ordering::Less => {
            let msg = format!(
                "You lost {}$ by having a lower card sum than the dealer ({} < {}).",
                bet, player_sum, dealer_sum,
            );
            println!("{}", msg.red());
            -(bet as i64)
        }
        Ordering::Equal => {
            let msg = format!(
                "You pushed by having the same card sum as the dealer ({}).",
                player_sum
            );
            println!("{}", msg.yellow());
            0
        }
        Ordering::Greater => {
            let msg = format!(
                "You won {}$ by having a higher card sum than the dealer ({} > {})!",
                bet, player_sum, dealer_sum,
            );
            println!("{}", msg.bright_green());
            bet as i64
        }
    }
}

const HEADER: &str = "=============== Blackjack ===============";
const OBJECTS_TO_SELL: &[&str] = &[
    "car",
    "house",
    "truck",
    "computer",
    "phone",
    "jewelry",
    "furniture",
    "pokemon cards",
    "lawnmower",
    "air fryer",
];

fn main() {
    let mut balance: i64 = 1000;

    loop {
        println!("{}", HEADER.bright_magenta());
        println!("Your balance: {}$", balance);
        let bet: u64 = get_bet(balance as u64);
        let money_gained: i64 = play(bet);
        balance += money_gained;
        if balance < 2 {
            break;
        }
        println!("{}", "[Press ENTER to restart]".white());
        std::io::stdin()
            .lock()
            .read_line(&mut String::new())
            .unwrap();
    }

    sleep_ms(723);

    let obj: &str = OBJECTS_TO_SELL.choose(&mut rng()).unwrap();
    let msg = format!("You gambled away all your money! Time to sell your {obj}...");
    println!("{}", msg.red());
}
