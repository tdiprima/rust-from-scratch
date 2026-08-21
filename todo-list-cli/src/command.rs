//! Parsing of raw user input into a typed `Command`.

/// Longest input accepted from the user. Guards against a pathological
/// paste turning into an unbounded task title.
const MAX_INPUT_LENGTH: usize = 1024;

/// A single user instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Add a task with the given title.
    Add(String),
    /// Mark the task at the given 1-based position as done.
    Done(usize),
    /// Print every task.
    List,
    /// Leave the program.
    Quit,
    /// Input that matched no known command.
    Unknown,
}

/// Parses one line of user input.
///
/// Input is untrusted: it is trimmed, length-checked, and every branch that
/// can produce an invalid command falls through to `Command::Unknown` rather
/// than panicking.
pub fn parse_command(input: &str) -> Command {
    let input = input.trim();

    if input.is_empty() || input.len() > MAX_INPUT_LENGTH {
        return Command::Unknown;
    }

    if input.eq_ignore_ascii_case("list") {
        return Command::List;
    }
    if input.eq_ignore_ascii_case("quit") {
        return Command::Quit;
    }
    if let Some(title) = strip_keyword(input, "add") {
        if title.is_empty() {
            return Command::Unknown;
        }
        return Command::Add(title.to_string());
    }
    if let Some(number) = strip_keyword(input, "done") {
        // parse::<usize>() rejects negatives, overflow, and non-digits for us.
        if let Ok(position) = number.parse::<usize>() {
            return Command::Done(position);
        }
    }

    Command::Unknown
}

/// Returns the trimmed remainder after a leading keyword and whitespace,
/// matching the keyword case-insensitively. `None` if the keyword is absent.
fn strip_keyword<'a>(input: &'a str, keyword: &str) -> Option<&'a str> {
    let (head, rest) = input.split_at_checked(keyword.len())?;
    if !head.eq_ignore_ascii_case(keyword) {
        return None;
    }
    // Require real whitespace so "adder foo" is not read as "add er foo".
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    Some(rest.trim())
}

#[cfg(test)]
mod tests {
    use super::{Command, MAX_INPUT_LENGTH, parse_command};

    #[test]
    fn parses_list_and_quit_case_insensitively() {
        assert_eq!(parse_command("list"), Command::List);
        assert_eq!(parse_command("  LIST  "), Command::List);
        assert_eq!(parse_command("Quit"), Command::Quit);
    }

    #[test]
    fn parses_add_with_title() {
        assert_eq!(
            parse_command("add Learn ownership"),
            Command::Add(String::from("Learn ownership"))
        );
        assert_eq!(
            parse_command("ADD   spaced   out  "),
            Command::Add(String::from("spaced   out"))
        );
    }

    #[test]
    fn add_without_title_is_unknown() {
        assert_eq!(parse_command("add"), Command::Unknown);
        assert_eq!(parse_command("add    "), Command::Unknown);
    }

    #[test]
    fn parses_done_with_number() {
        assert_eq!(parse_command("done 3"), Command::Done(3));
        assert_eq!(parse_command("DONE  12 "), Command::Done(12));
    }

    #[test]
    fn done_with_non_number_is_unknown() {
        assert_eq!(parse_command("done abc"), Command::Unknown);
        assert_eq!(parse_command("done -1"), Command::Unknown);
        assert_eq!(parse_command("done 1.5"), Command::Unknown);
        assert_eq!(parse_command("done"), Command::Unknown);
    }

    #[test]
    fn done_with_overflowing_number_is_unknown() {
        let too_big = "9".repeat(40);
        assert_eq!(parse_command(&format!("done {too_big}")), Command::Unknown);
    }

    #[test]
    fn empty_input_is_unknown() {
        assert_eq!(parse_command(""), Command::Unknown);
        assert_eq!(parse_command("   \t "), Command::Unknown);
    }

    #[test]
    fn keyword_must_be_followed_by_whitespace() {
        assert_eq!(parse_command("adder milk"), Command::Unknown);
        assert_eq!(parse_command("addmilk"), Command::Unknown);
        assert_eq!(parse_command("listing"), Command::Unknown);
    }

    #[test]
    fn oversized_input_is_rejected() {
        let long_title = "x".repeat(MAX_INPUT_LENGTH);
        assert_eq!(
            parse_command(&format!("add {long_title}")),
            Command::Unknown
        );
    }

    #[test]
    fn multibyte_input_does_not_panic() {
        // A shorter-than-keyword multibyte string must not split mid-character.
        assert_eq!(parse_command("é"), Command::Unknown);
        assert_eq!(parse_command("añadir tarea"), Command::Unknown);
        assert_eq!(
            parse_command("add café ☕"),
            Command::Add(String::from("café ☕"))
        );
    }
}
