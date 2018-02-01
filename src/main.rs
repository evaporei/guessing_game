extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the game!");
    println!("Choose a number between 1 and 10");

    let secret_number = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num > 10 || num < 1 {
                    println!("Choose a number between 1 and 10");
                    continue;
                }

                num
            },
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is too small!!"),
            Ordering::Greater => println!("Your number is too big!!"),
            Ordering::Equal => {
                println!("Matched!!Congrats :)");
                break;
            },
        }
    }
}
