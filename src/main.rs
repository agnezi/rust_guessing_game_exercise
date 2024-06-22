use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

// importing modules and traits

fn main() {
    println!("Guess a number");
    println!("Type a number from 0 to 10");
    let mut grn = thread_rng();
    let generated_number = grn.gen_range(0..=10);

    loop {
        let mut guess = String::new();

        // use &mut guess to avoid copying variables to another memory space, this will use a reference
        // use match to handle any possible error on user input
        match io::stdin().read_line(&mut guess) {
            Ok(..) => {
                println!("Your guess is -->{guess}");
            }
            Err(error) => println!("error {error}"),
        };

        // convert the guess string into u32 type to compare the number below
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        // use cmp to compare the generated number with the inputed one in u32 type
        // use Ordering enum to compare data
        match guess.cmp(&generated_number) {
            Ordering::Less => println!("Too small! Try a higher number"),
            Ordering::Greater => println!("Too big! Try a lower number"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
