// Copyright 2018 Christopher Simpkins
// Licensed under the MIT license

//! `commandlines` is a command line argument parsing library for the development of Rust command line interface (CLI) applications that follow the [POSIX / GNU conventions for command line arguments](https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html).
//!
//! It is in development and the API is not stable.  Please see the [source repository README.md page](https://github.com/chrissimpkins/commandlines-rust) for updates on the level of library support for the POSIX/GNU command line argument syntax.

pub mod parsers;

use std::collections::HashMap;
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
    /// HashMap of command line option definitions mapped as key=option:value=definition
    pub definitions: HashMap<String, String>,
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
        let definitions_hm = parsers::parse_definitions(&arguments);

        Command {
            argv: arguments_definition,
            argc: size_definition,
            executable: executable_definition.to_string(),
            options: vec_options,
            definitions: definitions_hm,
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
    ///    eprintln!("{}", "Missing arguments");
    /// }
    /// ```
    pub fn has_args(&self) -> bool {
        self.argv[1..].len() > 0
    }

    /// Returns a boolean for the question "Does the command include any definition options?"
    ///
    /// # Remarks
    /// A definition option is defined as a command line string that takes a short or long option format with an equal character that is used to indicate that a definition of the option follows.  They may take either of the following formats:
    /// * `-o=def`
    /// * `--option=def`
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// if c.has_definitions() {
    ///    // definitions were parsed in the command
    /// }
    /// ```
    pub fn has_definitions(&self) -> bool {
        self.definitions.len() > 0
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
    ///     // a `spam` argument was in the command
    /// }
    /// ```
    pub fn contains_arg(&self, needle: &str) -> bool {
        self.argv[1..].contains(&String::from(needle))
    }

    /// Returns a boolean for the question "Does the command include the definition option `needle`?"
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// if c.contains_definition("--spam") {
    ///     // command included a `--spam=[definition]` option
    /// }
    /// ```
    pub fn contains_definition(&self, needle: &str) -> bool {
        match self.definitions.get(&String::from(needle)) {
            Some(_) => true,
            None => false,
        }
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

    /// Returns Option<&String> definition for a key defined by `needle`
    ///
    /// Returns None if the option was not used in the command
    ///
    /// # Examples
    ///
    /// The following example demonstrates how to get the definition string for a command line option with the format `--name=[definition]`:
    ///
    /// ```
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// match c.get_definition_for("--name") {
    ///     Some(x) => println!("{}", *x),
    ///     None => eprintln!("{}", "Missing")
    /// };
    /// ```
    pub fn get_definition_for(&self, needle: &str) -> Option<&String> {
        self.definitions.get(&String::from(needle))
    }

    /// Returns `Option<&String>` for argument at index position i+1 for `needle` at index position i
    ///
    /// Returns None if `needle` is the last positional argument in the command
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new(std::env::args().collect());
    /// match c.get_argument_after("-o") {
    ///     Some(x) => println!("The argument after -o is {}", *x),
    ///     None => eprintln!("-o is the last positional argument in the command")
    /// }
    /// ```
    pub fn get_argument_after(&self, needle: &str) -> Option<&String> {
        for (index, value) in self.argv.iter().enumerate() {
            if value == needle {
                return self.argv.get(index + 1);
            }
        }

        None
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
    fn command_instantiation_definitions_field_single_def() {
        let c = Command::new(vec![
            "test".to_string(),
            "--something".to_string(),
            "--option=define".to_string(),
        ]);
        let mut expected_hm: HashMap<String, String> = HashMap::new();
        expected_hm.insert("--option".to_string(), "define".to_string());
        assert_eq!(c.definitions, expected_hm);
    }

    #[test]
    fn command_instantiation_definitions_field_multi_def() {
        let c = Command::new(vec![
            "test".to_string(),
            "--something".to_string(),
            "--option=define".to_string(),
            "--another=otherdef".to_string(),
            "--".to_string(),
            "--absent=true".to_string(),
        ]);
        let mut expected_hm: HashMap<String, String> = HashMap::new();
        expected_hm.insert("--option".to_string(), "define".to_string());
        expected_hm.insert("--another".to_string(), "otherdef".to_string());
        assert_eq!(c.definitions, expected_hm);
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
    fn command_method_has_definitions_true() {
        let c = Command::new(vec!["test".to_string(), "--opt=def".to_string()]);
        assert_eq!(c.has_definitions(), true);

        let c = Command::new(vec!["test".to_string(), "-o=d".to_string()]);
        assert_eq!(c.has_definitions(), true);
    }

    #[test]
    fn command_method_has_definitions_false() {
        let c = Command::new(vec!["test".to_string()]); // ignores the executable as not an argument
        assert_eq!(c.has_definitions(), false);
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
    fn command_method_contains_definition() {
        let c = Command::new(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
            "--option=definition".to_string(),
            "--another=deftwo".to_string(),
        ]);
        assert_eq!(c.contains_definition("--option"), true);
        assert_eq!(c.contains_definition("--another"), true);
        assert_eq!(c.contains_definition("--bogus"), false);
        assert_eq!(c.contains_definition("--help"), false);
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

    #[test]
    fn command_method_get_definition_for_def_present() {
        let c = Command::new(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
            "--option=definition".to_string(),
        ]);

        assert_eq!(
            c.get_definition_for("--option"),
            Some(&String::from("definition"))
        );
    }

    #[test]
    fn command_method_get_definition_for_def_absent() {
        let c = Command::new(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
        ]);

        assert_eq!(c.get_definition_for("--option"), None);
    }

    #[test]
    fn command_method_get_argument_after_arg_present() {
        let c = Command::new(vec![
            "test".to_string(),
            "-o".to_string(),
            "path".to_string(),
        ]);

        assert_eq!(c.get_argument_after("-o"), Some(&String::from("path")));
    }

    #[test]
    fn command_method_get_argument_after_arg_absent() {
        let c = Command::new(vec!["test".to_string(), "-o".to_string()]);

        assert_eq!(c.get_argument_after("-o"), None);
    }

}
