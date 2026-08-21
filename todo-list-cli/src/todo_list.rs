//! In-memory task collection and the pure logic that acts on it.
//!
//! Nothing here touches stdin/stdout, so every branch is testable.

use crate::command::Command;
use crate::task::Task;

/// What happened when a command was applied, ready to be printed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Lines to print to stdout.
    Message(Vec<String>),
    /// Print these lines, then exit the loop.
    Exit(Vec<String>),
}

impl Outcome {
    fn single(line: impl Into<String>) -> Outcome {
        Outcome::Message(vec![line.into()])
    }
}

/// The to-do list. Owns its tasks.
#[derive(Debug, Default)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    /// Applies one parsed command and reports what to show the user.
    pub fn apply(&mut self, command: Command) -> Outcome {
        match command {
            Command::Add(title) => self.add(title),
            Command::Done(position) => self.complete(position),
            Command::List => Outcome::Message(self.render()),
            Command::Quit => Outcome::Exit(vec![String::from("Bye!")]),
            Command::Unknown => {
                Outcome::single("  Unknown command. Try: add <task> | done <number> | list | quit")
            }
        }
    }

    /// Appends a task and confirms it.
    fn add(&mut self, title: String) -> Outcome {
        let confirmation = format!("  Added: {title}");
        self.tasks.push(Task::new(title));
        Outcome::single(confirmation)
    }

    /// Marks the task at a 1-based position done, or explains why it cannot.
    fn complete(&mut self, position: usize) -> Outcome {
        if position == 0 || position > self.tasks.len() {
            return Outcome::single("  Invalid task number. Use 'list' to see tasks.");
        }
        let task = &mut self.tasks[position - 1];
        task.complete();
        Outcome::single(format!("  Completed: {}", task.title))
    }

    /// Renders every task, or a hint when the list is empty.
    fn render(&self) -> Vec<String> {
        if self.tasks.is_empty() {
            return vec![String::from(
                "  No tasks yet. Use 'add <task>' to create one.",
            )];
        }
        self.tasks
            .iter()
            .enumerate()
            .map(|(index, task)| task.format_line(index + 1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Outcome, TodoList};
    use crate::command::Command;

    fn lines(outcome: &Outcome) -> &[String] {
        match outcome {
            Outcome::Message(lines) | Outcome::Exit(lines) => lines,
        }
    }

    fn task_count(list: &mut TodoList) -> usize {
        let outcome = list.apply(Command::List);
        match lines(&outcome) {
            [only] if only.starts_with("  No tasks yet") => 0,
            rendered => rendered.len(),
        }
    }

    #[test]
    fn new_list_is_empty() {
        let mut list = TodoList::new();
        assert_eq!(task_count(&mut list), 0);
    }

    #[test]
    fn list_on_empty_shows_hint() {
        let mut list = TodoList::new();
        let outcome = list.apply(Command::List);
        assert_eq!(
            lines(&outcome),
            ["  No tasks yet. Use 'add <task>' to create one."]
        );
    }

    #[test]
    fn add_then_list_numbers_from_one() {
        let mut list = TodoList::new();
        list.apply(Command::Add(String::from("Learn ownership")));
        list.apply(Command::Add(String::from("Write some Rust")));
        let outcome = list.apply(Command::List);
        assert_eq!(
            lines(&outcome),
            ["  1. [ ] Learn ownership", "  2. [ ] Write some Rust",]
        );
    }

    #[test]
    fn add_confirms_the_title() {
        let mut list = TodoList::new();
        let outcome = list.apply(Command::Add(String::from("Buy milk")));
        assert_eq!(lines(&outcome), ["  Added: Buy milk"]);
        assert_eq!(task_count(&mut list), 1);
    }

    #[test]
    fn duplicate_titles_are_allowed_and_independent() {
        let mut list = TodoList::new();
        list.apply(Command::Add(String::from("Same")));
        list.apply(Command::Add(String::from("Same")));
        list.apply(Command::Done(1));
        let outcome = list.apply(Command::List);
        assert_eq!(lines(&outcome), ["  1. [x] Same", "  2. [ ] Same"]);
    }

    #[test]
    fn done_marks_the_right_task() {
        let mut list = TodoList::new();
        list.apply(Command::Add(String::from("First")));
        list.apply(Command::Add(String::from("Second")));
        let outcome = list.apply(Command::Done(2));
        assert_eq!(lines(&outcome), ["  Completed: Second"]);
        let outcome = list.apply(Command::List);
        assert_eq!(lines(&outcome), ["  1. [ ] First", "  2. [x] Second"]);
    }

    #[test]
    fn done_zero_is_rejected() {
        let mut list = TodoList::new();
        list.apply(Command::Add(String::from("Only")));
        let outcome = list.apply(Command::Done(0));
        assert_eq!(
            lines(&outcome),
            ["  Invalid task number. Use 'list' to see tasks."]
        );
    }

    #[test]
    fn done_past_the_end_is_rejected() {
        let mut list = TodoList::new();
        list.apply(Command::Add(String::from("Only")));
        let outcome = list.apply(Command::Done(2));
        assert_eq!(
            lines(&outcome),
            ["  Invalid task number. Use 'list' to see tasks."]
        );
        let outcome = list.apply(Command::Done(usize::MAX));
        assert_eq!(
            lines(&outcome),
            ["  Invalid task number. Use 'list' to see tasks."]
        );
    }

    #[test]
    fn done_on_empty_list_is_rejected() {
        let mut list = TodoList::new();
        let outcome = list.apply(Command::Done(1));
        assert_eq!(
            lines(&outcome),
            ["  Invalid task number. Use 'list' to see tasks."]
        );
    }

    #[test]
    fn done_twice_stays_done() {
        let mut list = TodoList::new();
        list.apply(Command::Add(String::from("Once")));
        list.apply(Command::Done(1));
        list.apply(Command::Done(1));
        let outcome = list.apply(Command::List);
        assert_eq!(lines(&outcome), ["  1. [x] Once"]);
    }

    #[test]
    fn quit_signals_exit() {
        let mut list = TodoList::new();
        let outcome = list.apply(Command::Quit);
        assert!(matches!(outcome, Outcome::Exit(_)));
        assert_eq!(lines(&outcome), ["Bye!"]);
    }

    #[test]
    fn unknown_explains_the_commands() {
        let mut list = TodoList::new();
        let outcome = list.apply(Command::Unknown);
        assert_eq!(
            lines(&outcome),
            ["  Unknown command. Try: add <task> | done <number> | list | quit"]
        );
    }

    #[test]
    fn handles_many_tasks() {
        let mut list = TodoList::new();
        for number in 1..=10_000 {
            list.apply(Command::Add(format!("Task {number}")));
        }
        assert_eq!(task_count(&mut list), 10_000);
        list.apply(Command::Done(10_000));
        let outcome = list.apply(Command::List);
        assert_eq!(lines(&outcome).len(), 10_000);
        assert_eq!(lines(&outcome)[9_999], "  10000. [x] Task 10000");
    }
}
