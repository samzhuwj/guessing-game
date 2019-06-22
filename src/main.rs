extern crate rand;

use log::{trace, debug};
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use structopt::StructOpt;

fn main() {
    env_logger::init();
    let opt = Opt::from_args();
    for &lv in &opt.levels {
        println!("Given number range 0~{}", lv);
        guess_a_number((0, lv));
    }
}

#[derive(StructOpt)]
#[structopt(name="guessing_number_wasi")]
struct Opt {
    #[structopt(long="levels")]
    levels: Vec<u32>,
}

fn guess_a_number((lb, hb): (u32, u32)) {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(lb, hb + 1);
    trace!("Secret number: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess_str = String::new();
        io::stdin().read_line(&mut guess_str).expect("Failed to read line");
        debug!("Scanned string: {:?}", guess_str);

        let guess: u32 = match guess_str.trim().parse() {
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