use std::io;
use rand::Rng;
use std::cmp::Ordering;

/// Generates a random number between 1-100 (inclusive) and prompts the user for guessing the answer.
/// While the guessed number is incorrect, hints at the user if the guess is too small or big.
///
/// The game finishes when the user guesses correctly.
#[allow(dead_code)]
pub fn execute() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    loop {
        println!("Please, input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
