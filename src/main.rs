extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("GUESS THE NUMBER! (1-100)");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    let mut chances = 7;

    loop {
        if chances == 0 {
            println!("___ YOU LOOSE! ___");
            break;
        }

        println!("Please input your guess | Chance(s): {}", chances);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
                }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("___ YOU WIN, {}, Your score's: {} ___", secret_number, chances*10);
                break;
            }
        }

        chances = chances - 1;
    }
}