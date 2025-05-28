use std::io::Write;
use colored::Colorize;
use rand::{rng, Rng};
use crate::Action;

fn get_string_input(prompt: &str) -> String {
    print!("{} > ", format!("{prompt}").bright_cyan());
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not get line from stdin");
    input.trim().to_string()
}

pub fn get_action_input(double_allowed: bool) -> Action {
    loop {
        let input: String = get_string_input(if double_allowed {
            "Choose action: [s]tand | [h]it | [d]ouble"
        } else {
            "Choose action: [s]tand | [h]it"
        });

        match input.as_str() {
            "s" => return Action::Stand,
            "h" => return Action::Hit,
            "d" => if double_allowed {
                return Action::Double
            } else {
                println!("{}", "Doubling down is not allowed here!".bright_red());
            }
            _ => {
                println!("{}", "Invalid action input!".bright_red());
            }
        }
    }
}

pub fn get_bet(balance: u64) -> u64 {
    loop {
        let input: String = get_string_input("Choose your bet");
        match input.as_str() {
            "half" => return balance / 2,
            "all" => return balance,
            "idk" => return rng().random_range(2..=balance),
            _ => {},
        }

        let bet: u64 = match str::parse::<u64>(&input) {
            Ok(bet) => bet,
            Err(_) => {
                println!("{}", "Please provide a valid number!".bright_red());
                continue
            }
        };
        if bet > balance {
            println!("{}", "You cannot bet more than your balance!".bright_red());
            continue
        }
        if bet < 2 {
            println!("{}", "You must at least bet 2$!".bright_red());
            continue
        }
        return bet
    }
}

