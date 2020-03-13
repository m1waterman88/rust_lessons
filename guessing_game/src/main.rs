// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // Variables are immutable by default.
    // Add "mut" to create a mutable variable.
    let mut guess = String::new();

    // "::" for associated functions (static method)
    // "." for methods
    // "&" is a reference; reduce memory footprint
    // Reference are immutable by default too
    // So can't write "&guess"
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
