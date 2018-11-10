# Changelog

## v0.5.0

- added `Command::double_dash_argv` field that includes an ordered Vector of arguments that follow a double dash `--` command line idiom
- added `parsers::parse_double_dash_args()` function
- library documentation updates

## v0.4.0

- added `Command::contains_all_options()` method for `AND` logic multi-option testing
- added `Command::contains_any_option()` method for `OR` logic multi-option testing
- added `Command.first_arg` field with first positional argument definition
- added `Command.last_arg` field with last positional argument definition
- added `parsers::parse_first_arg()` function
- added `parsers::parse_last_arg()` function

## v0.3.0

- BACKWARDS INCOMPATIBLE: refactored `Command::new()` to use environment variables available in Rust `std::env::args()` on instantiation and removed the `Vec<String>` argument in this method.  This is the ideal approach for users, but eliminates the ability to test with mocked command line argument data
- created new `Command::new_with_vec()` method that instantiates a new `Command` struct with a `Vec<String>`.  This is not part of the public API and is intended for testing purposes
- added `Command::get_argument_at()` getter method for index-specific argument requests
- broadened unit test coverage
- added Makefile with source formatting, source testing, project publishing, and documentation build targets
- numerous documentation updates

## v0.2.1

- [bug] fix parsing of `-` as a command line option argument. It is not an option per POSIX guidelines.

## v0.2.0

- added`Command::get_argument_after` method for positional argument definition requests
- source code examples documentation updates

## v0.1.0

- initial release