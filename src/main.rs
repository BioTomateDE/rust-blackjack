use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::SliceRandom;
use rand::rng;
use colored::Colorize;


fn get_card_sum(cards: &Vec<u8>) -> u8 {
    let mut sum: u8 = 0;
    for card in cards {
        if *card == 11 && sum >= 11 {
            sum += 1;
        } else {
            sum += card;
        }
    }
    sum
}


enum Action {
    Stand,
    Hit,
    Double,
}

fn get_action_input(double_allowed: bool) -> Action {
    loop {
        if double_allowed {
            print!("{}", "Choose action: [s]tand | [h]it | [d]ouble > ".bright_cyan());
        } else {
            print!("{}", "Choose action: [s]tand | [h]it > ".bright_cyan());
        }
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Could not get action input");
        let input: &str = input.trim();
        match input {
            "s" => break Action::Stand,
            "h" => break Action::Hit,
            "d" => if double_allowed { 
                break Action::Double 
            } else {
                println!("{}", "Doubling down is not allowed here!".bright_red());
            }
            _ => {
                println!("{}", "Invalid action input!".bright_red());
            }
        }
    }
}


fn sleep_ms(ms: u32) {
    sleep(Duration::from_millis(u64::from(ms)));
}


fn play() {
    let mut deck: Vec<u8> = vec![
        2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11,
        2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11,
        2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11,
        2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11,
    ];
    deck.shuffle(&mut rng());

    let mut dealer_cards: Vec<u8> = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    let mut player_cards: Vec<u8> = vec![deck.pop().unwrap(), deck.pop().unwrap()];
    
    sleep_ms(230);
    println!("Dealer Upcard: {}\n", dealer_cards[0]);

    let mut player_sum: u8 = get_card_sum(&player_cards);
    let mut double_allowed: bool = true;
    let mut doubled_down: bool = false;

    while player_sum < 21 {
        println!("Your Sum: {} | Your Cards: {:?}", player_sum, player_cards);

        let action: Action = get_action_input(double_allowed);
        double_allowed = false;

        match action {
            Action::Stand => {
                println!("You stood on {player_sum}.");
                break
            },
            Action::Hit => player_cards.push(deck.pop().unwrap()),
            Action::Double => {
                player_cards.push(deck.pop().unwrap());
                doubled_down = true;
            }
        }
        player_sum = get_card_sum(&player_cards);
        sleep_ms(187);
    }

    if player_sum > 21 {
        println!("{}", format!("You busted with a sum of {player_sum}!").red());
        return;
    }
    
    sleep_ms(500);

    let mut dealer_sum: u8 = get_card_sum(&dealer_cards);
    while dealer_sum < 17 {
        dealer_cards.push(deck.pop().unwrap());
        dealer_sum = get_card_sum(&dealer_cards);
        print!("Dealer Sum: {} | Dealer Cards: {:?}\r", dealer_sum, dealer_cards);
        std::io::stdout().flush().unwrap();
        sleep_ms(806);
    }
    println!("Dealer Sum: {} | Dealer Cards: {:?}", dealer_sum, dealer_cards);
    
    if dealer_sum > 21 {
        println!("{}", format!("You won because the dealer busted with a sum of {dealer_sum}!").green());
        return;
    }

    sleep_ms(833);
    if player_sum > dealer_sum {
        println!("{}", format!("You won by having a higher sum than the dealer ({} > {})!", player_sum, dealer_sum).green());
    }
    else if player_sum == dealer_sum {
        println!("{}", format!("You pushed by having the same sum as the dealer ({}).", player_sum).yellow());
    }
    else {
        println!("{}", format!("You lost by having a lower sum than the dealer ({} < {}).", player_sum, dealer_sum).red());
    }
}



fn main() {
    loop {
        println!("{}", "=============== Blackjack ===============".bright_magenta());
        play();
        println!("{}", "[Press ENTER to restart]".white());
        let _ = std::io::stdin().lock().read_line(&mut String::new());
        // println!();
    }
}

