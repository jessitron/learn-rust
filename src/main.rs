use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nIt's time to GUESS THE NUMBER!");
    println!("******************************\n");
    let secret_number = rand::thread_rng().gen_range(1..=100); // ok that's amazing that it switched the type here because _later_ we compare it to a u32

    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Ugh, I can't handle anything but a number");

        println!("You guessed: {guess}");

        let result = match guess.cmp(&secret_number) {
            Ordering::Less => "Too small",
            Ordering::Equal => "CORRECT",
            Ordering::Greater => "Too big",
        };

        println!("{result}");
        if result == "CORRECT" {
            break;
        }
    }
}
