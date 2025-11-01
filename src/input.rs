use crate::Action;
use colored::Colorize;
use rand::{Rng, rng};
use std::io;
use std::io::Write;

fn get_string_input(prompt: &str) -> String {
    eprint!("{} > ", format!("{prompt}").bright_cyan());
    io::stderr().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn get_action_input(double_allowed: bool) -> Action {
    loop {
        let input: String = get_string_input(if double_allowed {
            "Choose action: [S]tand · [H]it · [D]ouble"
        } else {
            "Choose action: [S]tand · [H]it"
        });

        match input.to_ascii_lowercase().as_str() {
            "s" => return Action::Stand,
            "h" => return Action::Hit,
            "d" if double_allowed => return Action::Double,
            "d" => println!("{}", "Doubling down is not allowed here!".red()),
            _ => println!("{}", "Invalid action input!".red()),
        }
    }
}

pub fn get_bet(balance: u64) -> u64 {
    loop {
        match try_get_bet(balance) {
            Ok(bet) => return bet,
            Err(err) => println!("{}", err.red()),
        }
    }
}

fn try_get_bet(balance: u64) -> Result<u64, &'static str> {
    let input: String = get_string_input("Choose your bet");
    let bet: u64 = match input.as_str() {
        "half" => balance / 2,
        "all" => balance,
        "idk" => rng().random_range(2..=balance),
        _ => input
            .parse()
            .map_err(|_| "Please provide a valid number!")?,
    };

    if bet > balance {
        return Err("You cannot bet more than your balance!");
    }
    if bet < 2 {
        return Err("You must at least bet 2$!");
    }
    Ok(bet)
}
