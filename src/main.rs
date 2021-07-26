use rand::Rng;
use std::io;
use std::cmp::Ordering;

const GUESS_SIZE: usize = 3;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::with_capacity(GUESS_SIZE);

    println!("Guess a number. (1-100)");
        
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");
        
    let guess: u32 = guess.trim().parse().expect("Expecting valid unsigned 32bit integer");

    println!("Your guess: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Your number was over :("),
        Ordering::Less => println!("Your number was less :("),
        Ordering::Equal => {
            println!("Your number was equal, you win :D");
        }
    }
}