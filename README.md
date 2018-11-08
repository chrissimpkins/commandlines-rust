# commandlines for Rust

[![Build Status](https://travis-ci.org/chrissimpkins/commandlines-rust.svg?branch=master)](https://travis-ci.org/chrissimpkins/commandlines-rust) [![Build status](https://ci.appveyor.com/api/projects/status/1i4h0gsq82p4jmm9/branch/master?svg=true)](https://ci.appveyor.com/project/chrissimpkins/commandlines-rust/branch/master)

## About

`commandlines` is a command line argument parsing library for Rust command line interface application development.  The goal is to support most [POSIX/GNU program argument syntax conventions](https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html).

The project is in development and the library API is not stable.  Please see the developer documentation at https://docs.rs/commandlines.

## Current POSIX/GNU Argument Syntax Convention Support

### Available

- Arguments are options if they begin with a hyphen delimiter (`-`)
- Option names are single alphanumeric characters
- Options typically precede other non-option arguments
- The argument `--` terminates all options; any following arguments are treated as non-option arguments, even if they begin with a hyphen
- Options may be supplied in any order, or appear multiple times. The interpretation is left up to the particular application program
- Long options consist of `--` followed by a name made of alphanumeric characters and dashes. Option names are typically one to three words long, with hyphens to separate words
- To specify an argument for a long option, write ‘--name=value’. This syntax enables a long option to accept an argument that is itself optional
- Certain options require an argument. For example, the ‘-o’ command of the ld command requires an argument—an output file name


### Not Available Yet

- Multiple options may follow a hyphen delimiter in a single token if the options do not take arguments. Thus, `-abc` is equivalent to `-a -b -c`
- A token consisting of a single hyphen character is interpreted as an ordinary non-option argument. By convention, it is used to specify input from or output to the standard input and output streams.

### Do Not Plan to Support

- An option and its argument may or may not appear as separate tokens. (In other words, the whitespace separating them is optional.) Thus, ‘-o foo’ and ‘-ofoo’ are equivalent. 