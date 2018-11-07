// Copyright 2018 Christopher Simpkins
// Licensed under the MIT license

//! `commandlines` is a simple, functional command line argument parsing library for the development of Rust command line interface (CLI) applications.
//!
//! It is currently in development and is not stable for production use.

pub mod parsers;

use std::fmt;

/// A command line argument object
///
/// The `Command` struct defines fields that hold parsed command line argument data and provides methods that can be used to define the logic of a command line interface application.
///
/// # Examples
/// ## Instantiation
/// Instantiate a `Command` struct with a `Vec<String>` that is defined with `std::env::args().collect()`:
///
/// ```
/// extern crate commandlines;
///
/// use commandlines::Command;
///
/// let c = Command::new(std::env::args().collect());
/// ```
#[derive(Debug)]
pub struct Command {
    /// Vector of command line strings defined on instantiation
    pub argv: Vec<String>,
    /// number of strings in `Command.argv`
    pub argc: usize,
    /// executable at index position `0` of `Command.argv`
    pub executable: String,
    /// Vector of command line options in `Command.argv`
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
    /// Instantiates and returns a new `Command` object
    ///
    /// # Arguments
    ///
    /// * `arguments` - a `Vec<String>` of command line arguments
    ///
    /// # Remarks
    ///
    /// The command line arguments passed to the executable should be defined with `std::env::args().collect()`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate commandlines;
    ///
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// ```
    pub fn new(arguments: Vec<String>) -> Self {
        let arguments_definition = arguments.clone();
        let executable_definition = &arguments[0];
        let size_definition = arguments.len();
        let vec_options = parsers::parse_options(&arguments);

        Command {
            argv: arguments_definition,
            argc: size_definition,
            executable: executable_definition.to_string(),
            options: vec_options,
        }
    }

    /// Returns a boolean for the question "Does the command include any arguments?"
    ///
    /// # Remarks
    /// An argument is defined as a command line string after the executable. The executable at index position `0` in the `Vec<String>` returned by `std::env::args().collect()` is not part of this definition.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// if !c.has_args() {
    ///    eprintln!("{}", String::from("Missing arguments"));
    /// }
    /// ```
    pub fn has_args(&self) -> bool {
        self.argv[1..].len() > 0
    }

    /// Returns a boolean for the question "Does the command include any options?"
    ///
    /// # Remarks
    /// An option is defined as a command line string that starts with one or two hyphen characters. This definition includes standard long (e.g., `--longoption`) and short (e.g., `-s`) command line options.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// if c.has_options() {
    ///    // start application-specific option parsing logic
    /// }
    /// ```
    pub fn has_options(&self) -> bool {
        self.options.len() > 0
    }

    /// Returns a boolean for the question "Does the command include the argument string `needle`?" at any index
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// if c.contains_arg("spam") {
    ///     // you received spam somewhere in the command
    /// }
    /// ```
    pub fn contains_arg(&self, needle: &str) -> bool {
        self.argv[1..].contains(&String::from(needle))
    }

    /// Returns a boolean for the question "Does the command include the option string `needle`?" at any index
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// if c.contains_option("--help") {
    ///     // you have a standard request for help documentation
    /// }
    /// ````
    pub fn contains_option(&self, needle: &str) -> bool {
        self.options.contains(&String::from(needle))
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_instantiation_argv_field() {
        let c = Command::new(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.argv == vec!["test".to_string(), "--help".to_string()]);
    }

    #[test]
    fn command_instantiation_argc_field_one_arg() {
        let c = Command::new(vec!["test".to_string()]);
        assert!(c.argc == 1);
    }

    #[test]
    fn command_instantiation_argc_field_two_args() {
        let c = Command::new(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.argc == 2);
    }

    #[test]
    fn command_instantiation_executable_field() {
        let c = Command::new(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.executable == "test".to_string());
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
