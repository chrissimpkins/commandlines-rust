mod parsers;

use std::fmt;

#[derive(Debug)]
pub struct Command {
    pub argv: Vec<String>,
    pub argc: usize,
    pub options: Vec<String>
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
            options: vec_options
        }
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
}
