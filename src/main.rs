use std::io;

const GUESS_SIZE: usize = 5;

fn main() {
    let mut guess = String::with_capacity(GUESS_SIZE);

    io::stdin().read_line(&mut guess);
}