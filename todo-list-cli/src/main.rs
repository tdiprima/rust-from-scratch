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