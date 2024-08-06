use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing gaming!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read number!");

        let guess_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered: {guess_num}");

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("{}", "To small".red()),
            Ordering::Equal => {
                println!("{}", "Correct".green());
                break
            },
            Ordering::Greater => println!("{}", "To high".yellow()),
        }
    }
}