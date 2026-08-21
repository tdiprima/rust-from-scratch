# Lesson 1: Hello Rust

## Your First Rust Program: Hello World

Rust programs are typically managed with Cargo, which handles building, dependencies, and more.

### Step 1: Create a New Project

- Open a terminal and run:

  ```
  cargo new hello_rust
  cd hello_rust
  ```

- This creates a folder `hello_rust` with:
  - `Cargo.toml`: Your project's configuration file (like package.json).
  - `src/main.rs`: The main source file.

### Step 2: Write the Code

Open `src/main.rs` in your editor. It should already have this:

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()`: This is the entry point of every Rust program (like `main` in C++ or Java).
- `println!`: A macro (not a function — macros in Rust are like powerful code generators) that prints to the console. The `!` indicates it's a macro.

### Step 3: Build and Run

- In the terminal (from the `hello_rust` folder):

  ```
  cargo run
  ```

- This compiles your code (creates an executable) and runs it. Output: `Hello, world!`.
- Alternatively:
  - `cargo build`: Just compiles (creates a `target/debug/` folder with the binary).
  - `./target/debug/hello_rust`: Run the binary directly (on Linux/macOS; on Windows, it's `hello_rust.exe`).

If you make changes, `cargo run` will recompile automatically.

---

## How Rust Works: Key Concepts

Rust is "safe by default" thanks to its ownership system, which enforces rules at compile time to prevent bugs. It borrows ideas from functional programming (immutability) and systems languages (performance). No runtime garbage collection — memory is managed predictably.

### Core Principles

- **Safety**: Rust prevents memory errors (e.g., use-after-free) and data races (in concurrent code).
- **Performance**: As fast as C++ but safer.
- **Concurrency**: Built-in support for threads without fear of races.
- **Zero-Cost Abstractions**: Features like iterators are efficient — no overhead.

Key features explained with examples:

### Variables and Data Types

Variables are immutable by default (can't change after assignment). Use `mut` for mutable ones. Rust infers types but you can specify them.

```rust
fn main() {
    let x: i32 = 5;  // Immutable integer (i32 is 32-bit signed int)
    println!("x is {}", x);

    let mut y = 10;  // Mutable, type inferred as i32
    y = y + 1;
    println!("y is {}", y);

    // Common types: i32/u32 (signed/unsigned int), f64 (float), bool, char, &str (string slice)
    let name: &str = "Rust";
    let is_cool: bool = true;
}
```

- Run this by replacing your `main.rs` and using `cargo run`.

### Functions

Functions are defined with `fn`. They can return values (last expression is implicit return).

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}

fn main() {
    let sum = add(3, 4);
    println!("Sum: {}", sum);
}
```

### Control Flow

If-else, loops, etc., are similar to other languages but expressive.

```rust
fn main() {
    let num = 7;

    if num > 5 {
        println!("Greater than 5");
    } else {
        println!("Not greater");
    }

    // Loop with break (returns a value!)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;  // Breaks and returns 6
        }
    };
    println!("Loop result: {}", result);

    // For loop over range
    for i in 1..4 {
        println!("Iteration: {}", i);
    }
}
```

### Ownership, Borrowing, and Lifetimes

This is Rust's "secret sauce." Every value has an **owner**. When the owner goes out of scope, the value is dropped (freed). No manual memory management!

- **Ownership**: Passing a value transfers ownership.
- **Borrowing**: Use `&` to borrow (read-only) or `&mut` (mutable borrow). Only one mutable borrow at a time — no data races!
- **Lifetimes**: Ensure references don't outlive their data (marked with `'a`).

Example:

```rust
fn main() {
    let s1 = String::from("hello");  // s1 owns the string

    // Ownership transfer
    let s2 = s1;  // s1 is now invalid (moved to s2)
    // println!("{}", s1);  // Error! s1 no longer owns the value

    // Borrowing
    let len = calculate_length(&s2);  // Borrow s2 (read-only)
    println!("Length of '{}' is {}", s2, len);

    // Mutable borrow
    let mut s3 = String::from("hello");
    change(&mut s3);  // Borrow mutably
    println!("Changed: {}", s3);
}

fn calculate_length(s: &String) -> usize {  // Borrows, doesn't own
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
```

- If you try to use `s1` after moving, the compiler errors at build time — safety!
- This prevents bugs like dangling pointers.

### Error Handling

Rust uses `Result` and `Option` types instead of exceptions. No `null` — use `Option` for "maybe" values.

```rust
use std::fs::File;

fn main() {
    let file_result = File::open("hello.txt");

    match file_result {
        Ok(file) => println!("File opened!"),
        Err(error) => println!("Error: {}", error),
    }
}
```

### Modules and Crates

- Code is organized in modules (files or folders).
- Crates are packages (use Cargo to add dependencies in `Cargo.toml`, e.g., `rand = "0.8"` for random numbers, then `cargo build`).

&mdash; *Grok4*; Jul 16, 2025

<br>
