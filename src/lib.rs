//! `commandlines` is a simple, functional command line argument 
//! parsing library for the development of Rust command line interface 
//! (CLI) applications.

mod parsers;

use std::fmt;

#[derive(Debug)]
pub struct Command {
    pub argv: Vec<String>,
    pub argc: usize,
    pub options: Vec<String>,
}

// Traits

// Display trait
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut commandstring = String::new();
        for substring in &self.argv {
            commandstring.push_str(&substring[..]);
            commandstring.push_str(" ");
        }
        write!(f, "Command: '{}'", &commandstring[..].trim_right())
    }
}

// // Debug trait
// impl fmt::Debug for Command {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut commandstring = String::new();
//         for substring in &self.argv {
//             commandstring.push_str(&substring[..]);
//             commandstring.push_str(" ");
//         }
//         write!(f, "Command: '{}'", &commandstring[..].trim_right())
//     }
// }

// Methods
impl Command {
    pub fn new(arguments: Vec<String>) -> Self {
        let vec_size = arguments.len();
        let vec_options = parsers::parse_options(&arguments);

        Command {
            argv: arguments,
            argc: vec_size,
            options: vec_options,
        }
    }

    pub fn has_args(&self) -> bool {
        self.argv[1..].len() > 0
    }

    pub fn has_options(&self) -> bool {
        self.options.len() > 0
    }

    pub fn contains_arg(&self, needle: &str) -> bool {
        self.argv[1..].contains(&String::from(needle))
    }

    pub fn contains_option(&self, needle: &str) -> bool {
        self.options.contains(&String::from(needle))
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_instantiation_argv() {
        let c = Command::new(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.argv == vec!["test".to_string(), "--help".to_string()]);
    }

    #[test]
    fn command_instantiation_argc_one_arg() {
        let c = Command::new(vec!["test".to_string()]);
        assert!(c.argc == 1);
    }

    #[test]
    fn command_instantiation_argc_two_args() {
        let c = Command::new(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.argc == 2);
    }

    #[test]
    fn command_method_has_args_true() {
        let c = Command::new(vec!["test".to_string(), "--help".to_string()]);
        assert_eq!(c.has_args(), true);

        let c = Command::new(vec!["test".to_string(), "subcmd".to_string()]);
        assert_eq!(c.has_args(), true);
    }

    #[test]
    fn command_method_has_args_false() {
        let c = Command::new(vec!["test".to_string()]); // ignores the executable as not an argument
        assert_eq!(c.has_args(), false);
    }

    #[test]
    fn command_method_has_options_true() {
        let c = Command::new(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.has_options() == true);
    }

    #[test]
    fn command_method_has_options_false() {
        let c = Command::new(vec!["test".to_string(), "subcmd".to_string()]);
        assert!(c.has_options() == false);
    }

    #[test]
    fn command_method_contains_arg() {
        let c = Command::new(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
        ]);
        assert_eq!(c.contains_arg("subcmd"), true);
        assert_eq!(c.contains_arg("--help"), true);
        assert_eq!(c.contains_arg("bogus"), false);
        assert_eq!(c.contains_arg("test"), false); // executable not considered argument
    }

    #[test]
    fn command_method_contains_option() {
        let c = Command::new(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
        ]);
        assert_eq!(c.contains_option("--help"), true);
        assert_eq!(c.contains_option("--bogus"), false);
        assert_eq!(c.contains_option("help"), false); // must include the option indicator in string
    }

}
