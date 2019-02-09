
extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;
use std::process::exit;
use std::{thread, time};

#[derive(Clone)]
struct Guesser {
    tries: u32,
    secret_number: u32,
    game_type: String,
    spread: u32
}

fn main() {
    loop {
        guessing_game();
    }
}

fn new_secret_number()-> u32 {
    let sn :u32 = rand::thread_rng().gen_range(1, 101);
    return sn;
}

fn reveal_number(guesser: Guesser) {
    one_sec();
    println!("The secret number is {}", guesser.secret_number);
}

fn build_guesser() -> Guesser {
    let guesser = Guesser {
        tries: 3,
        secret_number: new_secret_number(),
        game_type: String::new(),
        spread: 0
    };
    return guesser;
}

fn escape(input: String) {
    let stop_commands = vec!["n", "N", "no", "No", "nope", "Nope", "exit", "stop", "quit", "end", "q"];
    if stop_commands.contains(&input.trim()) {
        exit(0);
    }
}

fn guessing_game() {
    let mut guesser = build_guesser();
    println!("Do you want to play best out of 3, or wager a spread? B|3/s");
    let mut game_type: String = String::new();
    io::stdin().read_line(&mut game_type).expect("What?");
    let esc: String = game_type.clone();
    escape(esc);
    let best_of_type = vec!["b", "B", "best", "Best", "3", ""];
    guesser.game_type = String::from(game_type.trim());
    if best_of_type.contains(&game_type.trim()) {
        best_of_game(guesser.clone());
        return;
    }
    let spread_type = vec!["s", "S", "spread", "Spread"];
    if spread_type.contains(&game_type.trim()) {
        spread_game(guesser);
    }
    end_game();
}

fn best_of_game(guesser: Guesser) {
    let guess = ask_game_input();
    let in_range = range_check(guess, 1, 100);
    if !in_range {
        decrement_tries(guesser);
        return;
    }
    one_sec();
    let (result, guesser) = guess_number(guess, guesser);
    if result {
        one_sec();
        ask_replay();
        return;
    }
    decrement_tries(guesser);
}

fn spread_game(mut guesser: Guesser) {
    guesser.spread = ask_spread_input();
    println!("If the number you guess is over or under by {} you win!", guesser.spread);
    one_sec();
    let guess = ask_game_input();
    let (result, guesser) = guess_number(guess, guesser);
    reveal_number(guesser);
    if result {
        println!("You guessed correctly!");
    } else {
        println!("You guessed wrong!");
    }
    one_sec();
    ask_replay();
}

fn ask_game_input() -> u32 {
    one_sec();
    let mut input = String::new();
    println!("Guess a number between 1 and 100...");
    io::stdin().read_line(&mut input).expect("What?");
    let esc: String = input.clone();
    escape(esc);
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("Error: {}", error.to_string());
            one_sec();
            println!("Please enter a number");
            return 0;
        },
    };
    return input;
}

fn ask_spread_input() -> u32 {
    one_sec();
    let mut input = String::new();
    let ask_spread = String::from("How close, over or under, do you think you can get to the secret number? (1-20)");
    println!("{}", &ask_spread);
    io::stdin().read_line(&mut input).expect(&ask_spread);
    let esc: String = input.clone();
    escape(esc);
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("{}", error.to_string());
            ask_spread_input();
            return 0;
        },
    };
    let in_range = range_check(input, 1, 20);
    if in_range {
        return input;
    } else {
        ask_spread_input();
        return 0;
    }
}

fn do_spread(guesser: Guesser, diff: u32)-> (bool, Guesser) {
    if 0 == guesser.spread || diff > guesser.spread {
        return (true, guesser);
    }
    return (true, guesser);
}

fn do_guess(guess: u32, guesser: Guesser, over_under: &str)-> (bool, Guesser) {
    let is_over: bool = over_under == "over";
    let diff = if is_over { guess - guesser.secret_number } else { guesser.secret_number - guess };
    one_sec();
    println!("You were {} by {}", over_under.to_string(), diff);
    return do_spread(guesser, diff);
}

fn guess_number(guess: u32, guesser: Guesser)-> (bool, Guesser) {
    println!("You guessed: {}", &guess);
    one_sec();
    match guess.cmp(&guesser.secret_number) {
        Ordering::Less => ({
            return do_guess(guess, guesser, "under");
        }),
        Ordering::Greater => ({
            return do_guess(guess, guesser, "over");
        }),
        Ordering::Equal => ({
            one_sec();
            println!("You guessed the secret number exactly!");
            return (true, guesser);
        }),
    }
}

fn range_check(input: u32, low: u32, high: u32)-> bool {
    if input > high || input < low {
        if input > 0 {
            one_sec();
            println!("The number you guessed is out of bounds.");
        }
        return false;
    }
    return true;
}

fn decrement_tries(mut guesser: Guesser) {
    guesser.tries = guesser.tries - 1;
    one_sec();
    if guesser.tries == 0 {
        println!("You're out of guesses");
        reveal_number(guesser);
        ask_replay();
    } else {
        let is_one: bool = guesser.tries == 1;
        let plural = ternary_str(is_one, "try", "tries");
        println!("You have {} {} remaining.", guesser.tries, plural);
        best_of_game(guesser);
    }
}

fn ask_replay() {
    one_sec();
    let play_again = vec!["y", "Y", "yes", "Yes", "yeah", "Yeah", "yup", "Yup", ""];
    let ask: String = String::from("Do you want to play again? Y/n");
    let mut input = String::new();
    println!("{}", ask);
    io::stdin().read_line(&mut input).expect(&ask);
    let esc: String = input.clone();
    escape(esc);
    if play_again.contains(&input.trim()) {
        main();
    }
}

fn end_game() {
    exit(0);
}

fn one_sec() {
    thread::sleep(time::Duration::from_millis(1000));
}

// TODO: move this to a tool crate
fn ternary_str(answer: bool, value1: &str, value2: &str)-> String {
    println!("{} and {}", value1, value2);
    if answer {
      return value1.to_string();
    } else {
      return value2.to_string();
    }
}
