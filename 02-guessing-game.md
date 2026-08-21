# Lesson 2: Guessing Game

A number-guessing game that ties together I/O, error handling, external crates, and pattern matching.

## Create the Project

```
cargo new guessing_game
cd guessing_game
```

## Add the `rand` Dependency

Add `rand` to `Cargo.toml` under `[dependencies]`:

```toml
[dependencies]
rand = "0.8.5"
```

Then run `cargo build` to download it.

## Write the Code

Edit `src/main.rs`:

```rust
use std::io;  // For input
use rand::Rng;  // Random number generation

fn main() {
    println!("Guess the number (1-100)!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Run It

```
cargo run
```

It generates a random number and lets you guess until you get it right.

## What This Demonstrates

- **Loops**: `loop` with `break` and `continue`
- **Input**: `io::stdin().read_line()` for user input
- **Error handling**: `match` on `Result` to handle parse failures gracefully
- **External crates**: Using `rand` from Cargo dependencies
- **Pattern matching**: `match` on `Ordering` enum variants
- **Variable shadowing**: Reusing the name `guess` for both `String` and `u32`

&mdash; *Grok4*; July 16, 2025

<br>
