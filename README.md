# rust-from-scratch

Hands-on Rust learning through working projects and annotated reference material.

## Learning a Systems Language Without the Overwhelm

Most Rust tutorials either skip the hard parts or dump everything on you at once — ownership, lifetimes, traits, async — before you've written a single useful program. That steep entry ramp discourages learners who just want to build something real.

## Learning by Doing, With Reference Material That Sticks

This repo pairs working Cargo projects with two plain-language reference documents. The projects give you something to run and modify immediately. The docs explain *why* Rust works the way it does, using analogies that connect to what you already know from Python or JavaScript.

## A Concrete Example

The guessing game project puts together I/O, error handling, external crates, and pattern matching in one runnable program:

```rust
use std::io;
use rand::Rng;

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
            std::cmp::Ordering::Less    => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Equal   => { println!("You win!"); break; }
        }
    }
}
```

## Usage

**Prerequisites:** Install Rust via [rustup](https://www.rust-lang.org/tools/install), which also installs Cargo.

```bash
# Verify your installation
rustc --version
cargo --version
```

**Run the Hello World project:**

```bash
cd hello_rust
cargo run
```

**Run the Guessing Game:**

```bash
cd guessing_game
cargo run
```

**Read the reference docs:**

| File | What it covers |
|---|---|
| `rust-tutorial.md` | Full beginner tutorial: setup, ownership, error handling, modules, Cargo |
| `syntax.md` | Core syntax explained as a natural language — variables, functions, loops, vectors, borrowing |

**Explore in the browser (no install needed):**  
Paste any snippet into the [Rust Playground](https://play.rust-lang.org) to run it immediately.

## What's in This Repo

```
rust-from-scratch/
├── hello_rust/          # Cargo project — Hello World
├── guessing_game/       # Cargo project — number guessing game (uses rand crate)
├── todo-list-cli/       # Cargo project — todo list tutorial
├── rust-tutorial.md     # Comprehensive beginner tutorial
└── syntax.md            # Syntax-focused reference with analogies
```
