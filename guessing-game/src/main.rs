use rand::{Rng, thread_rng};
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to process input");

        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too low");
            }
            Ordering::Greater => {
                println!("Too high");
            }
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
