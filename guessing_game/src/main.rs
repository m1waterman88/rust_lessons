// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hey! I'm thinking of a number between");
    println!("1 and 100. If you guess right, you get");
    println!("a prize!\nGO!");

    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    // println!("The secret number is {}", secret_number);
    
    loop {
        println!("Wha'da ya think?");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Remember the rules!");
                continue;
            }
        };
        
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Here's your prize...");
                println!("\\***** YOU. ARE. AWESOME. *****/");
                break;
            }
        }  
    }
}

