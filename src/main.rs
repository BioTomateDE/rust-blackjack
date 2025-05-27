use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::{IndexedRandom, SliceRandom};
use rand::{rng, Rng};
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


fn get_string_input(prompt: &str) -> String {
    print!("{} > ", format!("{prompt}").bright_cyan());
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not get line from stdin");
    input.trim().to_string()
}


fn get_action_input(double_allowed: bool) -> Action {
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


fn get_bet(balance: u64) -> u64 {
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


fn sleep_ms(ms: u32) {
    sleep(Duration::from_millis(u64::from(ms)));
}


/// returns the money gained (can be negative if lost)
fn play(mut bet: u64) -> i64 {
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
    let mut dealer_sum: u8 = get_card_sum(&dealer_cards);

    if player_sum == 21 {
        let payout: i64 = bet as i64 * 3 / 2;
        println!("Your Sum: {} | Your Cards: {:?}", player_sum, player_cards);
        println!("You won {payout}$ by getting a blackjack!");
        return payout
    }

    if dealer_sum == 21 {
        println!("Dealer Sum: {} | Dealer Cards: {:?}", dealer_sum, dealer_cards);
        println!("You lost {bet}$ because the dealer has a blackjack!");
        return -(bet as i64)
    }

    while player_sum <= 21 {
        println!("Your Sum: {} | Your Cards: {:?}", player_sum, player_cards);

        if player_sum == 21 {
            println!("Your Sum: {} | Your Cards: {:?}", player_sum, player_cards);
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
                println!("Your Sum: {} | Your Cards: {:?}", player_sum, player_cards);
                bet *= 2;
                break;
            }
        }
        player_sum = get_card_sum(&player_cards);
        sleep_ms(187);
    };

    if player_sum > 21 {
        println!("Your Sum: {} | Your Cards: {:?}", player_sum, player_cards);
        println!("{}", format!("You busted with a sum of {player_sum} and lost {bet}$!").red());
        return -(bet as i64);
    }

    sleep_ms(321);

    while dealer_sum < 17 {
        dealer_cards.push(deck.pop().unwrap());
        dealer_sum = get_card_sum(&dealer_cards);
        print!("Dealer Sum: {} | Dealer Cards: {:?}\r", dealer_sum, dealer_cards);
        std::io::stdout().flush().unwrap();
        sleep_ms(806);
    }
    println!("Dealer Sum: {} | Dealer Cards: {:?}", dealer_sum, dealer_cards);

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

