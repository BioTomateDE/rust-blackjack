mod card;
mod input;

use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::IndexedRandom;
use rand::rng;
use colored::Colorize;
use crate::card::{fmt_cards, gen_deck, get_card_sum, Card};
use crate::input::{get_action_input, get_bet};

enum Action {
    Stand,
    Hit,
    Double,
}


fn sleep_ms(ms: u32) {
    if std::env::var("NS").is_err() {
        sleep(Duration::from_millis(u64::from(ms)));
    }
}


/// returns the money gained (can be negative if lost)
fn play(mut bet: u64) -> i64 {
    let mut deck: Vec<Card> = gen_deck();

    let mut dealer_cards: Vec<Card> = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    let mut player_cards: Vec<Card> = vec![deck.pop().unwrap(), deck.pop().unwrap()];

    sleep_ms(230);
    println!("Dealer Upcard: {}\n", dealer_cards[0]);

    let mut player_sum: u8 = get_card_sum(&player_cards);
    let mut dealer_sum: u8 = get_card_sum(&dealer_cards);

    if player_sum == 21 {
        let payout: i64 = bet as i64 * 3 / 2;
        println!("Your Sum: {} | Your Cards: {}", player_sum, fmt_cards(&player_cards));
        println!("You won {payout}$ by getting a blackjack!");
        return payout
    }

    if dealer_sum == 21 {
        println!("Dealer Sum: {} | Dealer Cards: {}", dealer_sum, fmt_cards(&dealer_cards));
        println!("You lost {bet}$ because the dealer has a blackjack!");
        return -(bet as i64)
    }

    while player_sum <= 21 {
        println!("Your Sum: {} | Your Cards: {}", player_sum, fmt_cards(&player_cards));

        if player_sum == 21 {
            println!("Your Sum: {} | Your Cards: {}", player_sum, fmt_cards(&player_cards));
            break
        }

        let action: Action = get_action_input(player_cards.len() <= 2);

        match action {
            Action::Stand => {
                println!("You stood on {player_sum}.");
                break
            },
            Action::Hit => player_cards.push(deck.pop().unwrap()),
            Action::Double => {
                player_cards.push(deck.pop().unwrap());
                println!("Your Sum: {} | Your Cards: {}", player_sum, fmt_cards(&player_cards));
                bet *= 2;
                break;
            }
        }
        player_sum = get_card_sum(&player_cards);
        sleep_ms(187);
    };

    player_sum = get_card_sum(&player_cards);
    if player_sum > 21 {
        println!("Your Sum: {} | Your Cards: {}", player_sum, fmt_cards(&player_cards));
        println!("{}", format!("You busted with a sum of {player_sum} and lost {bet}$!").red());
        return -(bet as i64);
    }

    sleep_ms(321);

    while dealer_sum < 17 {
        dealer_cards.push(deck.pop().unwrap());
        dealer_sum = get_card_sum(&dealer_cards);
        print!("Dealer Sum: {} | Dealer Cards: {}\r", dealer_sum, fmt_cards(&dealer_cards));
        std::io::stdout().flush().unwrap();
        sleep_ms(806);
    }
    println!("Dealer Sum: {} | Dealer Cards: {}", dealer_sum, fmt_cards(&dealer_cards));

    if dealer_sum > 21 {
        println!("{}", format!("You won {bet}$ because the dealer busted with a sum of {dealer_sum}!").green());
        return bet as i64;
    }
    sleep_ms(533);

    if player_sum > dealer_sum {
        println!("{}", format!("You won {bet}$ by having a higher sum than the dealer ({} > {})!", player_sum, dealer_sum).green());
        bet as i64
    }
    else if player_sum == dealer_sum {
        println!("{}", format!("You pushed by having the same sum as the dealer ({}).", player_sum).yellow());
        0
    }
    else {
        println!("{}", format!("You lost {bet}$ by having a lower sum than the dealer ({} < {}).", player_sum, dealer_sum).red());
        -(bet as i64)
    }
}



fn main() {
    let mut balance: i64 = 1000;

    loop {
        println!("{}", "=============== Blackjack ===============".bright_magenta());
        println!("Your balance: {}$", balance);
        let bet: u64 = get_bet(balance as u64);     // cast is fine because the loop will break if balance <= 0
        let money_gained: i64 = play(bet);
        balance += money_gained;
        if balance <= 0 {
            break
        }
        println!("{}", "[Press ENTER to restart]".white());
        let _ = std::io::stdin().lock().read_line(&mut String::new());
    }

    sleep_ms(723);
    let objects_to_sell: [&str; 10] = ["car", "house", "truck", "computer", "phone", "jewelry", "furniture", "pokemon cards", "lawnmower", "air fryer"];
    let object_to_sell: &str = objects_to_sell.choose(&mut rng()).unwrap();
    println!("{}", format!("You gambled away all your money! Time to sell your {object_to_sell}...").red());
}

