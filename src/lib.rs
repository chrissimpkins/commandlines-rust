// Copyright 2018 Christopher Simpkins
// Licensed under the MIT license

//! `commandlines` is a command line argument parsing library for the development of Rust command line interface (CLI) applications that follow the [POSIX / GNU conventions for command line arguments](https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html).
//!
//! It is in development and the API is not stable.  Please see the [source repository README.md page](https://github.com/chrissimpkins/commandlines-rust) for updates on the level of library support for the POSIX/GNU command line argument syntax.

#![cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]

pub mod parsers;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;

/// A command line argument object
///
/// The `Command` struct defines fields that hold parsed command line argument data and provides methods that can be used to define the logic of a command line interface application.
///
/// # Examples
///
/// ## Instantiation
///
/// ```norun
/// extern crate commandlines;
///
/// use commandlines::Command;
///
/// def main() {
///     let c = Command::new();
/// }
/// ```
///
/// ## Debugging
///
/// The `Command` struct supports pretty-printed display of all parsed data fields to the standard output stream with the `{:#?}` formatting idiom:
///
/// ```
/// use commandlines::Command;
///
/// println!("{:#?}", Command::new());
/// ```
///
/// # Remarks
///
/// The Vector of command line arguments presented to the executable in `std::env::args().collect()` is used to define the `Command` struct fields.
///
/// See the documentation for the `Command` struct methods and fields to learn how to use the parsed data in your command line interface application logic.
#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    /// Vector of ordered command line arguments
    pub argv: Vec<String>,
    /// number of strings in `Command.argv`
    pub argc: usize,
    /// The executable path at index position `0` of `Command.argv`
    pub executable: String,
    /// Vector of command line options in `Command.argv`
    pub options: Vec<String>,
    /// HashMap of command line option definitions mapped as key=option:value=definition
    pub definitions: HashMap<String, String>,
    /// `Option<String>` of first positional argument to the executable. `None` if there are no arguments to the executable.
    pub first_arg: Option<String>,
    /// `Option<String>` of last positional argument to the executable. `None` if there are no arguments to the executable.
    pub last_arg: Option<String>,
    /// `Option<Vec<String>>` of ordered arguments that follow a double hyphen command line idiom. `None` if a double hyphen argument is not present or there are no arguments after the double hyphen argument.
    pub double_hyphen_argv: Option<Vec<String>>,
}

// Traits

// Display trait
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Command: '{}'", self.argv.join(" "))
    }
}

// Methods
impl Command {
    /// Instantiates and returns a new `Command` struct with the command line argument data in `std::env::args().collect()`
    ///
    /// # Remarks
    ///
    /// Instantiate a `Command` struct in the `main()` method of the `main.rs` file of your Rust executable project.
    ///
    /// # Examples
    ///
    /// ```norun
    /// extern crate commandlines;
    ///
    /// use commandlines::Command;
    ///
    /// def main() {
    ///     let c = Command::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Command::new_with_vec(std::env::args().collect())
    }

    // Instantiates and returns a new `Command` struct with mocked command line argument data that is passed in the `arguments` argument.
    //
    // # Arguments
    //
    // - arguments: (`Vec<String>`) - a Vector of ordered String items
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
    fn new_with_vec(arguments: Vec<String>) -> Self {
        let arguments_definition = arguments.to_owned();
        let executable_definition = &arguments[0];
        let size_definition = arguments.len();
        let vec_options = parsers::parse_options(&arguments);
        let definitions_hm = parsers::parse_definitions(&arguments);
        let first_arg_definition = parsers::parse_first_arg(&arguments);
        let last_arg_definition = parsers::parse_last_arg(&arguments);
        let double_hyphen_definition = parsers::parse_double_hyphen_args(&arguments);

        Command {
            argv: arguments_definition,
            argc: size_definition,
            executable: executable_definition.to_string(),
            options: vec_options,
            definitions: definitions_hm,
            first_arg: first_arg_definition,
            last_arg: last_arg_definition,
            double_hyphen_argv: double_hyphen_definition,
        }
    }

    /// Returns a boolean for the question "Does the command include any arguments to the executable?"
    ///
    /// # Remarks
    /// This method defines an argument as a command line string that occurs *after the executable path string located at index position `0`* of `Command.argv`. Please note that the executable path at index position `0` in the `Vec<String>` returned by `std::env::args().collect()` will always be present and is intentionally not part of this definition.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if !c.has_args() {
    ///    eprintln!("Missing arguments");
    /// }
    /// ```
    pub fn has_args(&self) -> bool {
        !self.argv[1..].is_empty()
    }

    /// Returns a boolean for the question "Does the command include any definition options?"
    ///
    /// # Remarks
    /// A definition option is defined as a command line string that takes a short or long option format with an equal symbol character that is used to indicate that an option definition string follows.  This library supports the following formats:
    ///
    /// - `--option=def`
    /// - `-o=def`
    ///
    /// The long format with two hyphens is specified in the GNU command line argument conventions.  The short format with a single hyphen is not specified in the POSIX or GNU guidelines.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.has_definitions() {
    ///    // definitions were parsed in the command
    /// }
    /// ```
    pub fn has_definitions(&self) -> bool {
        !self.definitions.is_empty()
    }

    /// Returns a boolean for the question "Does the command include any multi-option short syntax style option arguments?"
    ///
    /// # Remarks
    /// POSIX defines a short option style that uses a single hyphen delimiter with more than one option indicated by the individual characters defined in the argument string (e.g., `-hij` means that the command has the options `-h -i -j`).  This method provides support for determining whether a mops style option is present in the command string.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.has_mops() {
    ///     // at least one multi-option short syntax style argument was parsed in the command
    /// }
    /// ```
    pub fn has_mops(&self) -> bool {
        for arg in &self.options {
            if parsers::is_mops_option(&arg[..]) {
                return true;
            }
        }
        false
    }

    /// Returns a boolean for the question "Does the command include any options?"
    ///
    /// # Remarks
    /// An option is defined as a command line argument that starts with one or two hyphen characters. This definition includes standard long (e.g., `--longoption`) and short (e.g., `-s`) command line options.
    ///
    /// If you use POSIX multi-option short syntax style arguments (e.g., "-lmn" is used to indicate "-l -m -n") in your application, see the `Command::contains_mops()` method.  This method does not test against mops style command line arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.has_options() {
    ///    // start application-specific option parsing logic
    /// }
    /// ```
    pub fn has_options(&self) -> bool {
        !self.options.is_empty()
    }

    /// Returns a boolean for the question "Does the command include the argument string `needle`?" at any index
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
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
    /// let c = commandlines::Command::new();
    ///
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

    /// Returns a boolean for the question "Does the command include the option `needle` when the POSIX multi-option short syntax option style is used?"
    ///
    /// # Remarks
    /// The mops style defined by POSIX includes a single hyphen character followed by one or more alphanumeric characters in the Unicode Basic Latin set.  Each character represents a unique option switch.  For example, "-lmn" is used to indicate "-l -m -n".  This method tests against any short option formatted argument included in the command irrespective of the number of alphanumeric characters that are included.  The method does not test against long options (i.e., those that begin with two hyphens).
    ///
    /// If you do not use the mops option syntax in your application, use the `Command::contains_option()` method instead.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.contains_mops("-j") {
    ///     // the `j` switch was identified in a short format option on the command line
    /// }
    /// ```
    pub fn contains_mops(&self, needle: &str) -> bool {
        match parsers::parse_mops(&self.options) {
            Some(haystack) => haystack.contains(&String::from(needle)),
            None => false,
        }
    }

    /// Returns a boolean for the question "Does the command include the option string `needle` at any index?"
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.contains_option("--help") {
    ///     // you have a standard request for help documentation
    /// }
    /// ````
    pub fn contains_option(&self, needle: &str) -> bool {
        self.options.contains(&String::from(needle))
    }

    /// Returns a boolean for the question "Does the command include the option `needle` when the POSIX multi-option short syntax option style is used?"
    ///
    /// # Remarks
    /// The mops style defined by POSIX includes a single hyphen character followed by one or more alphanumeric characters in the Unicode Basic Latin set.  Each character represents a unique option switch.  For example, "-lmn" is used to indicate "-l -m -n".  This method tests against any short option formatted argument included in the command irrespective of the number of alphanumeric characters that are included.  The method does not test against long options (i.e., those that begin with two hyphens).
    ///
    /// If you do not use the mops option syntax in your application, use the `Command::contains_all_options()` method instead.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.contains_all_mops(vec!["-j", "-k", "-l"]) {
    ///     // the `j`, `k` and `l` switches were identified in short format options on the command line
    /// }
    /// ```
    pub fn contains_all_mops(&self, needle_vec: Vec<&str>) -> bool {
        let some_haystack = parsers::parse_mops(&self.options);
        if some_haystack.is_some() {
            let haystack = some_haystack.unwrap();
            for needle in needle_vec {
                if !haystack.contains(&String::from(needle)) {
                    return false;
                }
            }
        } else {
            return false; // there were no mops parsed in the command so there is no match
        }

        true
    }

    /// Returns a boolean for the question "Does the command include all of the option strings in `needle_vec` Vector?"
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.contains_all_options(vec!["--some", "--things"]) {
    ///     // implement whatever requires `some` && `things` condition
    /// }
    /// ```
    pub fn contains_all_options(&self, needle_vec: Vec<&str>) -> bool {
        for needle in needle_vec {
            if !self.options.contains(&String::from(needle)) {
                return false;
            }
        }
        true
    }

    /// Returns a boolean for the question "Does the command include the option `needle` when the POSIX multi-option short syntax option style is used?"
    ///
    /// # Remarks
    /// The mops style defined by POSIX includes a single hyphen character followed by one or more alphanumeric characters in the Unicode Basic Latin set.  Each character represents a unique option switch.  For example, "-lmn" is used to indicate "-l -m -n".  This method tests against any short option formatted argument included in the command irrespective of the number of alphanumeric characters that are included.  The method does not test against long options (i.e., those that begin with two hyphens).
    ///
    /// If you do not use the mops option syntax in your application, use the `Command::contains_any_options()` method instead.
    ///
    /// # Examples
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.contains_any_mops(vec!["-j", "-k", "-l"]) {
    ///     // the `j` or `k` or `l` switch was identified in short format options on the command line
    /// }
    /// ```
    pub fn contains_any_mops(&self, needle_vec: Vec<&str>) -> bool {
        let some_haystack = parsers::parse_mops(&self.options);
        if some_haystack.is_some() {
            let haystack = some_haystack.unwrap();
            for needle in needle_vec {
                if haystack.contains(&String::from(needle)) {
                    return true;
                }
            }
        } else {
            return false; // there were no mops parsed in the command so there is no match
        }

        false
    }

    /// Returns a boolean for the question "Does the command include any of the option strings in the `needle_vec` Vector?"
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.contains_any_option(vec!["-h", "--help"]) {
    ///     // received a help request with `-h` || `--help` condition
    /// }
    /// ```
    pub fn contains_any_option(&self, needle_vec: Vec<&str>) -> bool {
        for needle in needle_vec {
            if self.options.contains(&String::from(needle)) {
                return true;
            }
        }
        false
    }

    /// Returns boolean for the question "Do the command arguments to the executable match the argument strings and sequence in `needle_vec` Vector?"
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.contains_sequence(vec!["filter", "help"]) {
    ///     // the command sequence was identified as "[executable] filter help"
    /// }
    /// ```
    pub fn contains_sequence(&self, needle_vec: Vec<&str>) -> bool {
        // confirm that the request does not exceed the length of arguments in the command
        // subtract value of 1 for the executable which is excluded in this test
        if needle_vec.len() > (self.argv.len() - 1) {
            return false;
        }
        for (index, arg) in needle_vec.iter().enumerate() {
            if *arg != self.argv[index + 1] {
                return false;
            }
        }

        true
    }

    /// Returns `Option<Cow<'a, str>>` definition for a key defined by `needle`
    ///
    /// Returns `None` if the option was not used in the command
    ///
    /// # Examples
    ///
    /// The following example demonstrates how to get the definition string for a command line option with the format `--name=[definition]`:
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// match c.get_definition_for("--name") {
    ///     Some(x) => println!("The definition for --name is {}", x),
    ///     None => eprintln!("Missing")
    /// };
    /// ```
    pub fn get_definition_for<'a>(&'a self, needle: &str) -> Option<Cow<'a, str>> {
        if let Some(x) = self.definitions.get(&String::from(needle)) {
            return Some(Cow::Borrowed(x));
        }

        None
    }

    /// Returns `Option<Cow<'a, str>>` for argument at index position `i+1` for `needle` at index position `i`
    ///
    /// Returns `None` if `needle` is the last positional argument in the command
    ///
    /// # Remarks
    ///
    /// This method can be used to obtain space-delimited definition arguments that follow an option (e.g., `-o [filepath]`).
    ///
    /// # Examples
    ///
    /// For a command with the syntax `test -o [filepath]` the following can be used to get the filepath definition after the `-o` option:
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// match c.get_argument_after("-o") {
    ///     Some(x) => println!("The filepath definition after -o is {}", x),
    ///     None => eprintln!("-o is the last positional argument in the command")
    /// }
    /// ```
    pub fn get_argument_after<'a>(&'a self, needle: &str) -> Option<Cow<'a, str>> {
        for (index, value) in self.argv.iter().enumerate() {
            if value == needle {
                if let Some(x) = self.argv.get(index + 1) {
                    return Some(Cow::Borrowed(x));
                }
            }
        }

        None
    }

    /// Returns `Option<Cow<'a, str>>` for the argument at index position `needle`
    ///
    /// Returns `None` if `needle` is outside of the bounds of valid index values
    ///
    /// # Examples
    ///
    /// ```
    /// // example command = "test subcmd --option"
    /// let c = commandlines::Command::new();
    ///
    /// match c.get_argument_at(0) {
    ///     Some(x) => println!("The executable is {}", x),
    ///     None => eprintln!("Error")
    /// }
    ///
    /// match c.get_argument_at(1) {
    ///     Some(x) => println!("The first positional argument is {}", x),
    ///     None => eprintln!("There is no first positional argument")
    /// }
    /// ```
    pub fn get_argument_at<'a>(&'a self, needle: usize) -> Option<Cow<'a, str>> {
        if let Some(x) = self.argv.get(needle) {
            return Some(Cow::Borrowed(x));
        }

        None
    }

    /// Returns `Option<Cow<'a, str>>` for the first positional argument to the executable
    ///
    /// Returns `None` if there are no arguments to the executable
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// match c.get_argument_first() {
    ///     Some(x) => println!("The first positional argument is {}", x),
    ///     None => eprintln!("There are no arguments to the executable")
    /// }
    /// ```
    pub fn get_argument_first<'a>(&'a self) -> Option<Cow<'a, str>> {
        match &self.first_arg {
            Some(x) => Some(Cow::Borrowed(x)),
            None => None,
        }
    }

    /// Returns `Option<Cow<'a, str>>` for the last positional argument to the executable
    ///
    /// Returns `None` if there are no arguments to the executable
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// match c.get_argument_last() {
    ///     Some(x) => println!("The last positional argument is {}", x),
    ///     None => eprintln!("There are no arguments to the executable")
    /// }
    /// ```
    pub fn get_argument_last<'a>(&'a self) -> Option<Cow<'a, str>> {
        match &self.last_arg {
            Some(x) => Some(Cow::Borrowed(x)),
            None => None,
        }
    }

    /// Returns `Cow<'a, str>` for the executable
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// println!("{} v1.0.0", c.get_executable())
    ///
    /// ```
    pub fn get_executable<'a>(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(&self.executable)
    }

    /// Returns `Option<usize>` for index position of the argument `needle` in the `Command.argv` Vector
    ///
    /// Returns `None` if `needle` does not match a string in the Vector
    ///
    /// # Examples
    ///
    /// In the following example, the command is `test -o [filepath]`:
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// match c.get_index_of("-o") {
    ///     Some(x) => println!("The index position of -o is {}", x), // prints 1
    ///     None => eprintln!("The requested argument was not found")
    /// }
    /// ```
    pub fn get_index_of(&self, needle: &str) -> Option<usize> {
        self.argv.iter().position(|x| x == needle)
    }

    /// Returns boolean for the question "Is the command a help request with a `-h` or `--help` flag?"
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.is_help_request() {
    ///     // handle help request
    /// }
    /// ```
    pub fn is_help_request(&self) -> bool {
        self.options.contains(&"--help".to_string()) || self.options.contains(&"-h".to_string())
    }

    /// Returns boolean for the question "Is the command a version request with a `-v` or `--version` flag?"
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.is_version_request() {
    ///     // handle version request
    /// }
    /// ```
    pub fn is_version_request(&self) -> bool {
        self.options.contains(&"--version".to_string()) || self.options.contains(&"-v".to_string())
    }

    /// Returns boolean for the question "Is the command a usage request with a `--usage` flag?"
    ///
    /// # Examples
    ///
    /// ```
    /// let c = commandlines::Command::new();
    ///
    /// if c.is_usage_request() {
    ///     // handle usage request
    /// }
    /// ```
    pub fn is_usage_request(&self) -> bool {
        self.options.contains(&"--usage".to_string())
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_instantiation_partial_eq() {
        let c1 = Command::new_with_vec(vec!["test".to_string(), "arg".to_string()]);
        let c2 = Command::new_with_vec(vec!["test".to_string(), "arg".to_string()]);
        let c3 = Command::new_with_vec(vec!["test".to_string(), "else".to_string()]);
        assert!(c1 == c2);
        assert!(c1 != c3);
    }

    #[test]
    fn command_instantiation_environment_args_test() {
        let c = Command::new();
        assert!(!c.argv.is_empty()); // should always be a Vector with length 1 or more
    }

    #[test]
    fn command_instantiation_argv_field() {
        let c = Command::new_with_vec(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.argv == vec!["test".to_string(), "--help".to_string()]);
    }

    #[test]
    fn command_instantiation_argc_field_one_arg() {
        let c = Command::new_with_vec(vec!["test".to_string()]);
        assert!(c.argc == 1);
    }

    #[test]
    fn command_instantiation_argc_field_two_args() {
        let c = Command::new_with_vec(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.argc == 2);
    }

    #[test]
    fn command_instantiation_executable_field() {
        let c = Command::new_with_vec(vec!["test".to_string(), "--help".to_string()]);
        assert!(c.executable == "test");
    }

    #[test]
    fn command_instantiation_definitions_field_single_def() {
        let c = Command::new_with_vec(vec![
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
        let c = Command::new_with_vec(vec![
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
    fn command_instantiation_first_arg_field() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "--help".to_string(),
            "arg".to_string(),
        ]);
        assert_eq!(c.first_arg, Some(String::from("--help")));
    }

    #[test]
    fn command_instantiation_first_arg_field_executable_only() {
        let c = Command::new_with_vec(vec!["test".to_string()]);
        assert_eq!(c.first_arg, None);
    }

    #[test]
    fn command_instantiation_last_arg_field() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "--help".to_string(),
            "arg".to_string(),
        ]);
        assert_eq!(c.last_arg, Some(String::from("arg")));
    }

    #[test]
    fn command_instantiation_last_arg_field_executable_only() {
        let c = Command::new_with_vec(vec!["test".to_string()]);
        assert_eq!(c.last_arg, None);
    }

    #[test]
    fn command_instantiation_double_hyphen_argv() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "--something".to_string(),
            "--option=define".to_string(),
            "--another=otherdef".to_string(),
            "--".to_string(),
            "--double".to_string(),
            "lastpos".to_string(),
        ]);
        let expected_vec = vec![String::from("--double"), String::from("lastpos")];
        assert_eq!(c.double_hyphen_argv, Some(expected_vec));
    }

    #[test]
    fn command_instantiation_double_hyphen_argv_no_args() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "--something".to_string(),
            "--option=define".to_string(),
            "--another=otherdef".to_string(),
            "--".to_string(),
        ]);
        assert_eq!(c.double_hyphen_argv, None);
    }

    #[test]
    fn command_instantiation_double_hyphen_argv_no_double_hyphen() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "--something".to_string(),
            "--option=define".to_string(),
            "--another=otherdef".to_string(),
        ]);
        assert_eq!(c.double_hyphen_argv, None);
    }

    #[test]
    fn command_method_has_args_true() {
        let c = Command::new_with_vec(vec!["test".to_string(), "--help".to_string()]);
        assert_eq!(c.has_args(), true);

        let c = Command::new_with_vec(vec!["test".to_string(), "subcmd".to_string()]);
        assert_eq!(c.has_args(), true);
    }

    #[test]
    fn command_method_has_args_false() {
        let c = Command::new_with_vec(vec!["test".to_string()]); // ignores the executable as not an argument
        assert_eq!(c.has_args(), false);
    }

    #[test]
    fn command_method_has_definitions_true() {
        let c = Command::new_with_vec(vec!["test".to_string(), "--opt=def".to_string()]);
        assert_eq!(c.has_definitions(), true);

        let c = Command::new_with_vec(vec!["test".to_string(), "-o=d".to_string()]);
        assert_eq!(c.has_definitions(), true);
    }

    #[test]
    fn command_method_has_definitions_false() {
        let c = Command::new_with_vec(vec!["test".to_string()]); // ignores the executable as not an argument
        assert_eq!(c.has_definitions(), false);
    }

    #[test]
    fn command_has_mops_true() {
        let c = Command::new_with_vec(vec![
            String::from("command"),
            String::from("-lmn"),
            String::from("lastpos"),
        ]);
        assert_eq!(c.has_mops(), true);
    }

    #[test]
    fn command_has_mops_false() {
        let c = Command::new_with_vec(vec![
            String::from("command"),
            String::from("--long"),
            String::from("-o"),
            String::from("path"),
            String::from("lastpos"),
            String::from("--"),
            String::from("-"),
        ]);
        assert_eq!(c.has_mops(), false);
    }

    #[test]
    fn command_method_has_options_true() {
        let c = Command::new_with_vec(vec!["test".to_string(), "--help".to_string()]);
        assert_eq!(c.has_options(), true);
    }

    #[test]
    fn command_method_has_options_false() {
        let c = Command::new_with_vec(vec!["test".to_string(), "subcmd".to_string()]);
        assert_eq!(c.has_options(), false);
    }

    #[test]
    fn command_method_contains_arg() {
        let c = Command::new_with_vec(vec![
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
        let c = Command::new_with_vec(vec![
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
    fn command_method_contains_mops_true() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "-hij".to_string(),
            "-l".to_string(),
            "--option=definition".to_string(),
            "--another=deftwo".to_string(),
            "lastpos".to_string(),
        ]);
        assert_eq!(c.contains_mops("-h"), true);
        assert_eq!(c.contains_mops("-i"), true);
        assert_eq!(c.contains_mops("-j"), true);
        assert_eq!(c.contains_mops("-l"), true); // should pick up every short option, including those that are not mops formatted
        assert_eq!(c.contains_mops("-z"), false);
        assert_eq!(c.contains_mops("-o"), false);
        assert_eq!(c.contains_mops("-a"), false);
    }

    #[test]
    fn command_method_contains_mops_false() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--option=definition".to_string(),
            "--another=deftwo".to_string(),
            "lastpos".to_string(),
            "--".to_string(),
            "-hij".to_string(),
        ]);
        assert_eq!(c.contains_mops("-o"), false);
        assert_eq!(c.contains_mops("-a"), false);
        assert_eq!(c.contains_mops("-h"), false); // should ignore all options after a double hyphen idiom
        assert_eq!(c.contains_mops("-i"), false); // should ignore all options after a double hyphen idiom
        assert_eq!(c.contains_mops("-j"), false); // should ignore all options after a double hyphen idiom
    }

    #[test]
    fn command_method_contains_option() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
        ]);
        assert_eq!(c.contains_option("--help"), true);
        assert_eq!(c.contains_option("--bogus"), false);
        assert_eq!(c.contains_option("help"), false); // must include the option indicator in string
    }

    #[test]
    fn command_method_contains_all_mops_true() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "-hij".to_string(),
            "-l".to_string(),
            "--option=definition".to_string(),
            "--another=deftwo".to_string(),
            "lastpos".to_string(),
        ]);
        assert_eq!(c.contains_all_mops(vec!["-h", "-i", "-j", "-l"]), true);
    }

    #[test]
    fn command_method_contains_all_mops_false() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "-hij".to_string(),
            "-l".to_string(),
            "--option=definition".to_string(),
            "--another=deftwo".to_string(),
            "lastpos".to_string(),
        ]);
        assert_eq!(c.contains_all_mops(vec!["-z", "-h"]), false);
    }

    #[test]
    fn command_method_contains_all_mops_missing_mops() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--option=definition".to_string(),
            "--another=deftwo".to_string(),
            "lastpos".to_string(),
        ]);
        assert_eq!(c.contains_all_mops(vec!["-i", "-j"]), false);
    }

    #[test]
    fn command_method_contains_all_options() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
            "--more".to_string(),
        ]);
        assert_eq!(c.contains_all_options(vec!["--more", "--help"]), true);
        assert_eq!(c.contains_all_options(vec!["--help", "--bogus"]), false);
        assert_eq!(c.contains_all_options(vec!["--bogus"]), false);
        assert_eq!(c.contains_all_options(vec!["subcmd"]), false); // not an option, should not be included in test
    }

    #[test]
    fn command_method_contains_any_mops() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "-hij".to_string(),
            "-l".to_string(),
            "--option=definition".to_string(),
            "--another=deftwo".to_string(),
            "lastpos".to_string(),
        ]);
        assert_eq!(c.contains_any_mops(vec!["-z", "-t", "-h"]), true);
        assert_eq!(c.contains_any_mops(vec!["-z", "-t", "-l"]), true);
        assert_eq!(c.contains_any_mops(vec!["-z", "-t", "-a"]), false);
    }

    #[test]
    fn command_method_contains_any_mops_missing_mops() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--option=definition".to_string(),
            "--another=deftwo".to_string(),
            "lastpos".to_string(),
        ]);
        assert_eq!(c.contains_any_mops(vec!["-z", "-t", "-h"]), false);
    }

    #[test]
    fn command_method_contains_any_option() {
        let c1 = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
        ]);
        let c2 = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "-h".to_string(),
        ]);
        assert_eq!(c1.contains_any_option(vec!["--help", "-h"]), true);
        assert_eq!(c2.contains_any_option(vec!["--help", "-h"]), true);
        assert_eq!(c1.contains_any_option(vec!["--bogus", "-t"]), false);
        assert_eq!(c1.contains_any_option(vec!["subcmd", "bogus"]), false);
    }

    #[test]
    fn command_method_contains_sequence() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "subsubcmd".to_string(),
        ]);
        assert_eq!(c.contains_sequence(vec!["subcmd", "subsubcmd"]), true);
        assert_eq!(c.contains_sequence(vec!["subcmd"]), true);
        assert_eq!(c.contains_sequence(vec!["subsubcmd", "subcmd"]), false); // wrong order fails
        assert_eq!(c.contains_sequence(vec!["bogus", "subsubcmd"]), false); // any invalid string fails
        assert_eq!(c.contains_sequence(vec!["subcmd", "bogus"]), false); // any invalid string fails
        assert_eq!(
            c.contains_sequence(vec!["subcmd", "subsubcmd", "toomuchinfo"]),
            false
        ); // fail with too many argument requests c/w command that was entered
        assert_eq!(c.contains_sequence(vec!["bogus", "bogus", "bogus"]), false); // fail all invalid strings
        assert_eq!(
            c.contains_sequence(vec!["subcmd", "subsubcmd", "more", "evenmore", "lotsmore"]),
            false
        );
    }

    #[test]
    fn command_method_get_definition_for_def_present() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
            "--option=definition".to_string(),
        ]);

        assert_eq!(
            c.get_definition_for("--option"),
            Some(Cow::Borrowed("definition"))
        );
    }

    #[test]
    fn command_method_get_definition_for_def_absent() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "subcmd".to_string(),
            "--help".to_string(),
        ]);

        assert_eq!(c.get_definition_for("--option"), None);
    }

    #[test]
    fn command_method_get_argument_after_arg_present() {
        let c = Command::new_with_vec(vec![
            "test".to_string(),
            "-o".to_string(),
            "path".to_string(),
        ]);

        assert_eq!(c.get_argument_after("-o"), Some(Cow::Borrowed("path")));
    }

    #[test]
    fn command_method_get_argument_after_arg_absent() {
        let c = Command::new_with_vec(vec!["test".to_string(), "-o".to_string()]);

        assert_eq!(c.get_argument_after("-o"), None);
    }

    #[test]
    fn command_method_get_argument_after_missing_needle_arg() {
        let c = Command::new_with_vec(vec!["test".to_string(), "-o".to_string()]);

        assert_eq!(c.get_argument_after("bogus"), None);
    }

    #[test]
    fn command_method_get_argument_at() {
        let c = Command::new_with_vec(vec!["test".to_string(), "-o".to_string()]);

        assert_eq!(c.get_argument_at(0), Some(Cow::Borrowed("test"))); // zero indexed request
        assert_eq!(c.get_argument_at(1), Some(Cow::Borrowed("-o"))); // valid index
        assert_eq!(c.get_argument_at(10), None); // invalid index
    }

    #[test]
    fn command_method_get_argument_first() {
        let c1 = Command::new_with_vec(vec!["test".to_string(), "-o".to_string()]);
        let c2 = Command::new_with_vec(vec![
            "test".to_string(),
            "-o".to_string(),
            "more".to_string(),
        ]);
        let c3 = Command::new_with_vec(vec!["test".to_string(), "first".to_string()]);
        let c4 = Command::new_with_vec(vec!["test".to_string(), "--help".to_string()]);
        let c5 = Command::new_with_vec(vec![
            "test".to_string(),
            "--help".to_string(),
            "more".to_string(),
        ]);
        let c6 = Command::new_with_vec(vec!["test".to_string()]);

        assert_eq!(c1.get_argument_first(), Some(Cow::Borrowed("-o"))); // short option
        assert_eq!(c2.get_argument_first(), Some(Cow::Borrowed("-o"))); // short option with additional args
        assert_eq!(c3.get_argument_first(), Some(Cow::Borrowed("first"))); // subcommand style argument
        assert_eq!(c4.get_argument_first(), Some(Cow::Borrowed("--help"))); // long option
        assert_eq!(c5.get_argument_first(), Some(Cow::Borrowed("--help"))); // long option with additional args
        assert_eq!(c6.get_argument_first(), None);
    }

    #[test]
    fn command_method_get_argument_last() {
        let c1 = Command::new_with_vec(vec!["test".to_string(), "-o".to_string()]);
        let c2 = Command::new_with_vec(vec![
            "test".to_string(),
            "-o".to_string(),
            "more".to_string(),
        ]);
        let c3 = Command::new_with_vec(vec!["test".to_string(), "first".to_string()]);
        let c4 = Command::new_with_vec(vec!["test".to_string(), "--help".to_string()]);
        let c5 = Command::new_with_vec(vec![
            "test".to_string(),
            "--help".to_string(),
            "more".to_string(),
        ]);
        let c6 = Command::new_with_vec(vec!["test".to_string()]);

        assert_eq!(c1.get_argument_last(), Some(Cow::Borrowed("-o"))); // short option
        assert_eq!(c2.get_argument_last(), Some(Cow::Borrowed("more"))); // short option followed by LP arg
        assert_eq!(c3.get_argument_last(), Some(Cow::Borrowed("first"))); // subcommand style argument
        assert_eq!(c4.get_argument_last(), Some(Cow::Borrowed("--help"))); // long option
        assert_eq!(c5.get_argument_last(), Some(Cow::Borrowed("more"))); // long option followed by LP arg
        assert_eq!(c1.get_argument_last(), Some(Cow::Borrowed("-o")));
        assert_eq!(c6.get_argument_last(), None);
    }

    #[test]
    fn command_method_get_executable() {
        let c1 = Command::new_with_vec(vec!["test".to_string(), "-o".to_string()]);
        let c2 = Command::new_with_vec(vec!["~/user/path/to/test".to_string(), "-o".to_string()]);
        assert_eq!(c1.get_executable(), Cow::Borrowed("test"));
        assert_eq!(c2.get_executable(), Cow::Borrowed("~/user/path/to/test"));
    }

    #[test]
    fn command_method_get_index_of() {
        let c = Command::new_with_vec(vec!["test".to_string(), "-o".to_string()]);

        assert_eq!(c.get_index_of("test"), Some(0));
        assert_eq!(c.get_index_of("-o"), Some(1));
        assert_eq!(c.get_index_of("missing"), None);
    }

    #[test]
    fn command_method_is_help_request() {
        let c1 = Command::new_with_vec(vec!["test".to_string(), "-h".to_string()]);
        let c2 = Command::new_with_vec(vec!["test".to_string(), "--help".to_string()]);
        let c3 = Command::new_with_vec(vec!["test".to_string(), "-help".to_string()]);
        let c4 = Command::new_with_vec(vec!["test".to_string(), "--h".to_string()]);
        let c5 = Command::new_with_vec(vec!["test".to_string(), "--else".to_string()]);
        let c6 = Command::new_with_vec(vec!["test".to_string(), "h".to_string()]);
        let c7 = Command::new_with_vec(vec!["test".to_string(), "help".to_string()]);

        assert_eq!(c1.is_help_request(), true);
        assert_eq!(c2.is_help_request(), true);
        assert_eq!(c3.is_help_request(), false);
        assert_eq!(c4.is_help_request(), false);
        assert_eq!(c5.is_help_request(), false);
        assert_eq!(c6.is_help_request(), false);
        assert_eq!(c7.is_help_request(), false);
    }

    #[test]
    fn command_method_is_version_request() {
        let c1 = Command::new_with_vec(vec!["test".to_string(), "-v".to_string()]);
        let c2 = Command::new_with_vec(vec!["test".to_string(), "--version".to_string()]);
        let c3 = Command::new_with_vec(vec!["test".to_string(), "-version".to_string()]);
        let c4 = Command::new_with_vec(vec!["test".to_string(), "--v".to_string()]);
        let c5 = Command::new_with_vec(vec!["test".to_string(), "--else".to_string()]);
        let c6 = Command::new_with_vec(vec!["test".to_string(), "v".to_string()]);
        let c7 = Command::new_with_vec(vec!["test".to_string(), "version".to_string()]);

        assert_eq!(c1.is_version_request(), true);
        assert_eq!(c2.is_version_request(), true);
        assert_eq!(c3.is_version_request(), false);
        assert_eq!(c4.is_version_request(), false);
        assert_eq!(c5.is_version_request(), false);
        assert_eq!(c6.is_version_request(), false);
        assert_eq!(c7.is_version_request(), false);
    }

    #[test]
    fn command_method_is_usage_request() {
        let c1 = Command::new_with_vec(vec!["test".to_string(), "--usage".to_string()]);
        let c2 = Command::new_with_vec(vec!["test".to_string(), "-u".to_string()]);
        let c3 = Command::new_with_vec(vec!["test".to_string(), "-usage".to_string()]);
        let c4 = Command::new_with_vec(vec!["test".to_string(), "--else".to_string()]);
        let c5 = Command::new_with_vec(vec!["test".to_string(), "usage".to_string()]);

        assert_eq!(c1.is_usage_request(), true);
        assert_eq!(c2.is_usage_request(), false);
        assert_eq!(c3.is_usage_request(), false);
        assert_eq!(c4.is_usage_request(), false);
        assert_eq!(c5.is_usage_request(), false);
    }
}
