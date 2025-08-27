//! konsole Library for reading from the stdin
//! A rust-native version of java utility
//!

use super::*;
use std::io::Cursor;

#[test]
fn test_read_line_with_cursor() {
    let cursor = Cursor::new("rusty\n");
    let name = _read_line("Enter a name: ", cursor);
    assert_eq!(name, "rusty");
}

#[test]
fn test_read_line_with_stdin() {
    let name = read_line("Enter a name: ");
    assert_eq!(name, Some("".to_string()));
}
