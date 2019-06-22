extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    guess_a_number((1, 100));
}

fn guess_a_number((lb, hb): (u32, u32)) {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(lb, hb + 1);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The input is not a number! Please input a valid number");
                continue;
            },
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}