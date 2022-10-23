use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new(); // Growable utf-8 bit of text

        io::stdin()
            .read_line(&mut guess) // references are immutable by default
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed {guess}");

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
