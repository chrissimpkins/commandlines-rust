# Changelog

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