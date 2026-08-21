//! Task data type: a single to-do item.

/// A single to-do item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub title: String,
    pub done: bool,
}

impl Task {
    /// Creates a new task in the pending state.
    pub fn new(title: String) -> Task {
        Task { title, done: false }
    }

    /// Marks the task as completed.
    pub fn complete(&mut self) {
        self.done = true;
    }

    /// Renders the task as a numbered list line, e.g. `  1. [x] Learn ownership`.
    pub fn format_line(&self, position: usize) -> String {
        let check = if self.done { "x" } else { " " };
        format!("  {}. [{}] {}", position, check, self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::Task;

    #[test]
    fn new_task_starts_pending() {
        let task = Task::new(String::from("Learn ownership"));
        assert_eq!(task.title, "Learn ownership");
        assert!(!task.done);
    }

    #[test]
    fn complete_marks_done() {
        let mut task = Task::new(String::from("Write some Rust"));
        task.complete();
        assert!(task.done);
    }

    #[test]
    fn complete_is_idempotent() {
        let mut task = Task::new(String::from("Repeat"));
        task.complete();
        task.complete();
        assert!(task.done);
    }

    #[test]
    fn format_line_shows_pending_and_done() {
        let mut task = Task::new(String::from("Buy milk"));
        assert_eq!(task.format_line(1), "  1. [ ] Buy milk");
        task.complete();
        assert_eq!(task.format_line(1), "  1. [x] Buy milk");
    }

    #[test]
    fn format_line_handles_unicode_title() {
        let task = Task::new(String::from("café ☕"));
        assert_eq!(task.format_line(7), "  7. [ ] café ☕");
    }
}
