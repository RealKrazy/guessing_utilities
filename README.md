# Guessing utilities

This crate provides various guessing utilities for working with guesses in (0..101) number range.
Check the [https://docs.rs/guessing_utils/1.0.2/guessing_utils/](documentation) for more.

## Fully functional example

```rust
use guessing_utils;
use guessing_utils::Guess;

use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = guessing_utils::gen_random();

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guessing_utils::Guess::parse(&guess) {
            Ok(val) => val,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());

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
```