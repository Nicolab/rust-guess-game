// Guess game - https://github.com/Nicolab/rust-guess-game

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // attempts
    let mut attempts = 0u32;

    // Debug only
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("\nʘ‿ʘ Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("(⊙_◎) Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("(⊙_◎) Oops! Bad number...");
                continue;
            }
        };

        attempts += 1;
        println!("#{} - Your number: {}", attempts, guess);

        // match the user input
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("▲ (Too small!)"),
            Ordering::Greater => println!("▼ (Too big!)"),
            Ordering::Equal => {
                println!("ʘ‿ʘ");
                println!("(｡◕‿◕｡) You win! (｡◕‿◕｡)");
                break;
            }
        }

        println!("----------------------------");
    }
}
