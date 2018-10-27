extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Please input your guess...");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let stop_commands = vec!["stop", "quit", "end", "q"];

    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if stop_commands.contains(&guess.trim()) {
                    break;
                }
                println!("Please enter a number");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        println!("The secret number is {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => ({
                let diff = secret_number - guess;
                println!("You were under by {}", diff);
            }),
            Ordering::Greater => ({
                let diff = guess - secret_number;
                println!("You were over by {}", diff);
            }),
            Ordering::Equal => {
                println!("You guessed the secret number exactly!");
                break;
            },
        }
    }
}
