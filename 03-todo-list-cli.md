# Lesson 3: Todo List CLI

A hands-on guide to Rust's core language concepts, followed by a project walkthrough where you build a to-do list CLI app.

---

Part 1 covers core language concepts: variables/mutability, types, functions, control flow, ownership, borrowing, structs, enums, pattern matching, Option/Result, vectors, String vs &str, traits, and error propagation with ?.

Part 2 walks through building a CLI to-do list app that ties all those concepts together — structs, enums with data, pattern matching, vectors, borrowing, and error handling. Includes stretch goals for extending it.

Run `cargo init` inside todo-list-cli/ then paste the code to get started.

Then `cargo run`.

---

## Part 1: Core Language Concepts

### Variables and Mutability

Variables are **immutable by default**. You must opt in to mutability with `mut`.

```rust
let name = "Alice";       // immutable — cannot reassign
let mut score = 0;        // mutable — can reassign
score = 10;               // works fine
```

This prevents accidental mutation. If the compiler sees you never mutate a `mut` variable, it warns you.

**Shadowing** lets you redeclare a variable with the same name, even with a different type:

```rust
let input = "42";
let input: i32 = input.parse().unwrap(); // shadows the string with an integer
```

### Data Types

Rust is statically typed. The compiler infers types most of the time, but you can annotate explicitly.

**Scalar types:**

| Type | Examples |
|------|----------|
| Integer | `i32`, `u64`, `i8`, `usize` |
| Float | `f32`, `f64` |
| Boolean | `bool` |
| Character | `char` (Unicode scalar, 4 bytes) |

**Compound types:**

```rust
// Tuple — fixed size, mixed types
let point: (f64, f64) = (3.5, 7.2);
let x = point.0; // access by index

// Array — fixed size, same type
let days = ["Mon", "Tue", "Wed"];
let first = days[0];
```

For growable collections, use `Vec<T>` (covered below).

### Functions

Functions use `fn`, snake_case names, and explicit parameter/return types:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon = this is the return value
}
```

**Key detail:** the last expression in a function body (without a semicolon) is the return value. Adding a semicolon turns it into a statement that returns `()` (unit/void).

### Control Flow

```rust
// if/else — no parentheses around the condition
if temperature > 100 {
    println!("Too hot");
} else if temperature < 0 {
    println!("Too cold");
} else {
    println!("Just right");
}

// if as an expression
let status = if score > 50 { "pass" } else { "fail" };

// loop — runs forever until you break
loop {
    // do work
    break; // exit the loop
}

// while
while count < 10 {
    count += 1;
}

// for — the most common loop
for number in 1..=5 {
    println!("{number}"); // prints 1 through 5
}

for item in &my_vector {
    println!("{item}");
}
```

### Ownership — Rust's Big Idea

Ownership is how Rust manages memory without a garbage collector. Three rules:

1. Every value has exactly one **owner** (a variable).
2. When the owner goes out of scope, the value is **dropped** (freed).
3. Ownership can be **moved** — after a move, the original variable is invalid.

```rust
let name = String::from("Alice");
let greeting = name; // ownership moves to `greeting`
// println!("{name}"); // COMPILE ERROR — `name` no longer owns the data
println!("{greeting}"); // works
```

**Why this matters:** no double-frees, no dangling pointers, no data races — all enforced at compile time.

#### Borrowing and References

Instead of moving ownership, you can **borrow** with references:

```rust
fn print_length(text: &String) {
    println!("Length: {}", text.len());
}

let name = String::from("Alice");
print_length(&name); // borrow — name is still valid after this
println!("{name}");   // works fine
```

**Mutable references** let you modify borrowed data, but Rust enforces: only one mutable reference at a time (prevents data races).

```rust
fn add_exclamation(text: &mut String) {
    text.push('!');
}

let mut message = String::from("Hello");
add_exclamation(&mut message);
println!("{message}"); // "Hello!"
```

### Structs

Group related data into a named type:

```rust
struct Task {
    title: String,
    done: bool,
}

let task = Task {
    title: String::from("Buy groceries"),
    done: false,
};

println!("{}: {}", task.title, task.done);
```

**Methods** are defined in an `impl` block:

```rust
impl Task {
    fn new(title: &str) -> Task {
        Task {
            title: String::from(title),
            done: false,
        }
    }

    fn complete(&mut self) {
        self.done = true;
    }
}

let mut task = Task::new("Buy groceries");
task.complete();
```

### Enums and Pattern Matching

Enums represent a value that can be one of several variants:

```rust
enum Command {
    Add(String),
    Done(usize),
    List,
    Quit,
}
```

**`match`** is Rust's pattern matching — it must be exhaustive (handle every variant):

```rust
match command {
    Command::Add(title) => println!("Adding: {title}"),
    Command::Done(index) => println!("Completing task {index}"),
    Command::List => println!("Listing tasks"),
    Command::Quit => println!("Goodbye"),
}
```

#### Option and Result

Two enums you will use constantly:

```rust
// Option — a value that might not exist
let maybe: Option<i32> = Some(42);
let nothing: Option<i32> = None;

match maybe {
    Some(value) => println!("Got {value}"),
    None => println!("Nothing here"),
}

// Result — an operation that might fail
let parsed: Result<i32, _> = "42".parse();

match parsed {
    Ok(number) => println!("Parsed: {number}"),
    Err(error) => println!("Failed: {error}"),
}
```

**`.unwrap()`** extracts the value but panics on `None`/`Err`. Fine for tutorials and prototypes — in production, handle errors explicitly.

### Vectors

`Vec<T>` is a growable array:

```rust
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

// or use the vec! macro
let mut names = vec!["Alice", "Bob"];
names.push("Charlie");

// iterate
for name in &names {
    println!("{name}");
}

// access by index (panics if out of bounds)
println!("{}", names[0]);

// safe access
match names.get(99) {
    Some(name) => println!("{name}"),
    None => println!("No such index"),
}
```

### String vs &str

Rust has two main string types:

- **`String`** — owned, heap-allocated, growable. You use this when you need to own or modify string data.
- **`&str`** — a borrowed reference to string data. String literals (`"hello"`) are `&str`.

```rust
let owned: String = String::from("hello");  // you own this data
let borrowed: &str = "hello";               // points to data baked into the binary

// convert between them
let to_owned: String = borrowed.to_string();
let to_borrowed: &str = &owned;
```

Rule of thumb: accept `&str` in function parameters, return `String` when the function creates new string data.

### Traits

Traits define shared behavior (similar to interfaces):

```rust
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Task {
    fn summarize(&self) -> String {
        let status = if self.done { "done" } else { "pending" };
        format!("{} [{}]", self.title, status)
    }
}
```

Common traits you will use from the standard library: `Display`, `Debug`, `Clone`, `Iterator`.

**Derive macros** auto-implement common traits:

```rust
#[derive(Debug, Clone)]
struct Task {
    title: String,
    done: bool,
}

println!("{:?}", task); // Debug output
```

### Error Handling with `?`

The `?` operator propagates errors up to the caller — much cleaner than nested `match`:

```rust
use std::io;

fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // if this fails, return the error
    Ok(input.trim().to_string())
}
```

The function must return `Result` (or `Option`) to use `?`.

---

## Part 2: Project Walkthrough — To-Do List CLI

Build a command-line to-do list that lets you add tasks, mark them done, list them, and quit.

### Create the Project

```bash
cd ~/blah/todo-list-cli
cargo init
```

This creates:

```
todo-list-cli/
├── Cargo.toml
└── src/
    └── main.rs
```

### Write the Code

Replace the contents of `src/main.rs` with:

```rust
use std::io;
use std::io::Write;

struct Task {
    title: String,
    done: bool,
}

impl Task {
    fn new(title: String) -> Task {
        Task { title, done: false }
    }
}

enum Command {
    Add(String),
    Done(usize),
    List,
    Quit,
    Unknown,
}

fn parse_command(input: &str) -> Command {
    let input = input.trim();

    if input.eq_ignore_ascii_case("list") {
        return Command::List;
    }
    if input.eq_ignore_ascii_case("quit") {
        return Command::Quit;
    }
    if let Some(title) = input.strip_prefix("add ") {
        let title = title.trim();
        if title.is_empty() {
            return Command::Unknown;
        }
        return Command::Add(title.to_string());
    }
    if let Some(number) = input.strip_prefix("done ") {
        if let Ok(index) = number.trim().parse::<usize>() {
            return Command::Done(index);
        }
    }

    Command::Unknown
}

fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("  No tasks yet. Use 'add <task>' to create one.");
        return;
    }
    for (index, task) in tasks.iter().enumerate() {
        let check = if task.done { "x" } else { " " };
        println!("  {}. [{}] {}", index + 1, check, task.title);
    }
}

fn read_input(prompt: &str) -> Result<String, io::Error> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    println!("=== To-Do List ===");
    println!("Commands: add <task> | done <number> | list | quit");
    println!();

    loop {
        let input = match read_input("> ") {
            Ok(input) => input,
            Err(error) => {
                eprintln!("Error reading input: {error}");
                continue;
            }
        };

        match parse_command(&input) {
            Command::Add(title) => {
                println!("  Added: {title}");
                tasks.push(Task::new(title));
            }
            Command::Done(number) => {
                if number == 0 || number > tasks.len() {
                    println!("  Invalid task number. Use 'list' to see tasks.");
                    continue;
                }
                tasks[number - 1].done = true;
                println!("  Completed: {}", tasks[number - 1].title);
            }
            Command::List => {
                list_tasks(&tasks);
            }
            Command::Quit => {
                println!("Bye!");
                break;
            }
            Command::Unknown => {
                println!("  Unknown command. Try: add <task> | done <number> | list | quit");
            }
        }
    }
}
```

### What Each Piece Demonstrates

| Code section | Rust concept |
|---|---|
| `struct Task` | Structs, associated functions (`new`) |
| `enum Command` | Enums with data variants |
| `parse_command` | Pattern matching, `if let`, string methods |
| `list_tasks(&[Task])` | Borrowing a slice, iterators, `enumerate` |
| `read_input` | `Result`, the `?` operator, error handling |
| `match parse_command(...)` | Exhaustive pattern matching |
| `tasks: Vec<Task>` | Vectors, ownership of heap data |
| `tasks[number - 1].done = true` | Mutable access to vector elements |

### Build and Run

```bash
cargo run
```

Sample session:

```
=== To-Do List ===
Commands: add <task> | done <number> | list | quit

> add Learn ownership
  Added: Learn ownership
> add Write some Rust
  Added: Write some Rust
> list
  1. [ ] Learn ownership
  2. [ ] Write some Rust
> done 1
  Completed: Learn ownership
> list
  1. [x] Learn ownership
  2. [ ] Write some Rust
> quit
Bye!
```

### Stretch Goals

Once this works, try extending it:

- **Delete a task** — add a `Remove(usize)` variant and use `tasks.remove(index)`
- **Save to a file** — write tasks to a `.txt` or `.json` file so they survive between runs (look into `std::fs`)
- **Edit a task** — add an `Edit(usize, String)` variant to rename a task
- **Priority levels** — add a priority field to `Task` using an enum (`Low`, `Medium`, `High`)

---

## Quick Reference

| Concept | One-liner |
|---|---|
| Immutability | Variables immutable by default; add `mut` to opt in |
| Ownership | Every value has one owner; when the owner is dropped, so is the value |
| Borrowing | `&T` for read access, `&mut T` for write access — no data races |
| Structs | Named data types with fields; add methods in `impl` blocks |
| Enums | Types with multiple variants; can carry data |
| `match` | Exhaustive pattern matching on enums, Options, Results |
| `Option<T>` | `Some(value)` or `None` — Rust's null replacement |
| `Result<T, E>` | `Ok(value)` or `Err(error)` — Rust's error handling |
| `?` operator | Propagates errors to the caller; cleaner than nested matches |
| `Vec<T>` | Growable array; owns its elements |
| Traits | Shared behavior across types; like interfaces |
| `String` vs `&str` | Owned vs borrowed string data |

&mdash; *Opus 4.6;* Aug 17, 2026

<br>
