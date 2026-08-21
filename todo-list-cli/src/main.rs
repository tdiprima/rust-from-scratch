//! To-do list CLI: read a command, apply it, print the result, repeat.

mod command;
mod task;
mod todo_list;

use std::io;
use std::io::Write;
use std::process::ExitCode;

use command::parse_command;
use todo_list::{Outcome, TodoList};

const PROMPT: &str = "> ";
const USAGE: &str = "Commands: add <task> | done <number> | list | quit";

fn main() -> ExitCode {
    print_banner();

    let mut tasks = TodoList::new();

    loop {
        match read_input(PROMPT) {
            Ok(Some(input)) => {
                let outcome = tasks.apply(parse_command(&input));
                if print_outcome(&outcome) {
                    return ExitCode::SUCCESS;
                }
            }
            // EOF (Ctrl-D or a closed pipe): stop rather than spin forever.
            Ok(None) => {
                println!();
                println!("Bye!");
                return ExitCode::SUCCESS;
            }
            Err(error) => {
                eprintln!("Error reading input: {error}");
                return ExitCode::FAILURE;
            }
        }
    }
}

fn print_banner() {
    println!("=== To-Do List ===");
    println!("{USAGE}");
    println!();
}

/// Prints an outcome. Returns true when the program should stop.
fn print_outcome(outcome: &Outcome) -> bool {
    match outcome {
        Outcome::Message(lines) => {
            print_lines(lines);
            false
        }
        Outcome::Exit(lines) => {
            print_lines(lines);
            true
        }
    }
}

fn print_lines(lines: &[String]) {
    for line in lines {
        println!("{line}");
    }
}

/// Reads one trimmed line. `Ok(None)` means end of input.
fn read_input(prompt: &str) -> Result<Option<String>, io::Error> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;
    if bytes_read == 0 {
        return Ok(None);
    }
    Ok(Some(input.trim().to_string()))
}
