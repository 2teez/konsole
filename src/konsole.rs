//! konsole Library for reading from the stdin
//! A rust-native version of java utility
//!

use std::io::{self, BufRead, Read, Write};

///
/// read_line function
/// ```
/// use std::io::Cursor;
/// use konsole::konsole::_read_line;
///
/// let cursor = Cursor::new("rusty\n");
/// let name = _read_line("Enter a name: ", cursor);
/// assert_eq!(name, "rusty".to_string());
/// ```
///
pub fn read_line(msg: &str) -> Option<String> {
    Some(_read_line(msg, std::io::stdin().lock()))
}

#[doc(hidden)]
pub fn _read_line<R: Read + BufRead>(msg: &str, mut reader: R) -> String {
    print!("{}", msg);
    io::stdout().flush().expect("can't flush standard output.");
    let mut line = String::new();
    reader.read_line(&mut line).expect("can't read line");
    line = line.trim().to_string();
    line
}

// TODO
pub fn read_passowrd(msg: &str) -> Vec<char> {
    todo!("to be implemented not using external lib")
}

#[cfg(test)]
mod unittest;
