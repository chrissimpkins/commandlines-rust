// Copyright 2018 Christopher Simpkins
// Licensed under the MIT license

//! Command line string operating system path utilities

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Returns an immutable `Path` for the String or string slice reference `pathstring`.
pub fn make_path_from<S: AsRef<OsStr>>(pathstring: &S) -> &Path {
    Path::new(pathstring)
}

/// Returns a mutable `PathBuf` for the String or string slice reference `pathstring`.
pub fn make_mut_path_from<S: AsRef<OsStr>>(pathstring: &S) -> PathBuf {
    PathBuf::from(pathstring)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_make_path_from() {
        let path_longpath = "this/is/a/path.txt";
        let path_shortpath = "path.txt";
        let path_emptypath = "";
        let path_string_longpath = String::from("this/is/a/path.txt");
        let path_string_shortpath = String::from("path.text");
        let path_string_emptypath = String::from("");

        let parent1 = make_path_from(&path_longpath).parent();
        let parent2 = make_path_from(&path_shortpath).parent();
        let parent3 = make_path_from(&path_emptypath).parent();
        let parent4 = make_path_from(&path_string_longpath).parent();
        let parent5 = make_path_from(&path_string_shortpath).parent();
        let parent6 = make_path_from(&path_string_emptypath).parent();

        assert_eq!(parent1, Some(Path::new("this/is/a")));
        assert_eq!(parent2, Some(Path::new("")));
        assert_eq!(parent3, None);
        assert_eq!(parent4, Some(Path::new("this/is/a")));
        assert_eq!(parent5, Some(Path::new("")));
        assert_eq!(parent6, None);
    }

    #[test]
    fn path_make_mut_path_from() {
        let path_longpath = "this/is/a/path.txt";
        let path_shortpath = "path.txt";
        let path_emptypath = "";
        let path_string_longpath = String::from("this/is/a/path.txt");
        let path_string_shortpath = String::from("path.text");
        let path_string_emptypath = String::from("");

        let mut pb1 = make_mut_path_from(&path_longpath);
        pb1.set_file_name("bar.txt");
        let mut pb2 = make_mut_path_from(&path_shortpath);
        pb2.set_file_name("bar.txt");
        let mut pb3 = make_mut_path_from(&path_emptypath);
        pb3.set_file_name("bar.txt");
        let mut pb4 = make_mut_path_from(&path_string_longpath);
        pb4.set_file_name("bar.txt");
        let mut pb5 = make_mut_path_from(&path_string_shortpath);
        pb5.set_file_name("bar.txt");
        let mut pb6 = make_mut_path_from(&path_string_emptypath);
        pb6.set_file_name("bar.txt");

        assert_eq!(pb1, PathBuf::from("this/is/a/bar.txt"));
        assert_eq!(pb2, PathBuf::from("bar.txt"));
        assert_eq!(pb3, PathBuf::from("bar.txt"));
        assert_eq!(pb4, PathBuf::from("this/is/a/bar.txt"));
        assert_eq!(pb5, PathBuf::from("bar.txt"));
        assert_eq!(pb6, PathBuf::from("bar.txt"));
    }

}
