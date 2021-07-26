use std::io;

const STR_ALLOCATION_SIZE: usize = 5;

fn main() {
    loop {
        let mut some_text: String = String::with_capacity(STR_ALLOCATION_SIZE);

        io::stdin()
            .read_line(&mut some_text)
            .expect("Could not read CLI input.");

        println!("Parotting input: {}", some_text);
    }
}