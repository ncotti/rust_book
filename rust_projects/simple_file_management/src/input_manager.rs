use crate::str_literals::*;
use std::io::{BufRead, Error, Write};
use std::path::Path;

/// Reads from an input buffer all characters until a "\n" is received
///
/// @param:
///     buffered_input: Every struct that implements the `Read` trait can be used if passed as
///     argument to a `std::io:BufReader::new()`.
///
/// @return:
///     A Result with `Ok(String)` with the line read or `Err(std::io::Error)`.
pub fn get_user_input(buffered_input: &mut impl BufRead) -> Result<String, Error> {
    let mut line = String::new();
    buffered_input.read_line(&mut line)?;
    Ok(line)
}

/// Reads a filepath from a read buffer (normally stdin) and blocks until it's a valid file.
///
/// @param:
///     * buffered_input: Any struct that implements the `Read` trait and was used as argument with
///     `std::io::BufReader::new()`.
///
///     * output: Any output (normally stdout).
///
/// @return:
///     An Option with a smart pointer to a `std::path::Path` struct, holding the valid file path.
pub fn get_filepath(
    buffered_input: &mut impl BufRead,
    output: &mut impl Write,
) -> Option<Box<Path>> {
    loop {
        output.write(ASKING_FOR_USER_FILE).expect("");
        output.flush().expect("");

        let line = get_user_input(buffered_input).unwrap();
        let line = line.trim();

        if line == "q" {
            break;
        }

        let file_path = Path::new(&line);

        if file_path.is_file() {
            return Some(file_path.into());
        } else {
            output.write(b"Not a file.\n").expect("");
            continue;
        }
    }
    return None;
}
