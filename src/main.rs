use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!(
        "\nIt's time to GUESS THE NUMBER!"
    );
    println!(
        "******************************\n"
    );

    println!("Please input your guess: ");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    let as_str = ToString::to_string("{secret_number}");

    let result = match guess.cmp(&as_str) {
        Ordering::Less => "Too small",
        Ordering::Equal => "CORRECT",
        Ordering::Greater => "Too big",
    };

    println!("{result}");


}
