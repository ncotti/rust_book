//! # Simple File Management
//!
//! This module contains a full example of managing Stdin and Stdout.
mod input_manager;
mod str_literals;
mod user;

use std::fs;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::path::Path;

use input_manager::get_user_input;
use str_literals::*;
use user::User;

/// Function menu explanation
pub fn menu(input: &mut impl Read, output: &mut impl Write) -> Result<(), Error> {
    let mut buffered_input = BufReader::new(input);

    loop {
        output
            .write(WELCOME_MSG)
            .expect("A write to stdout should never fail.");

        let line = input_manager::get_user_input(&mut buffered_input).unwrap();

        match line.chars().next().unwrap_or('5') {
            '1' => {
                let file_path: Option<Box<Path>> =
                    input_manager::get_filepath(&mut buffered_input, output);
                if file_path.is_none() {
                    continue;
                }

                let users = list_users(&file_path.unwrap());

                for user in users.iter() {
                    println!("{}", user);
                }
            }
            '2' => {
                let file_path = input_manager::get_filepath(&mut buffered_input, output);
                if file_path.is_none() {
                    continue;
                }

                let file_path = file_path.unwrap();

                //let users = list_users(file_path.unwrap());

                output.write(b"Please, enter the name: ").expect("");
                let name = String::from(get_user_input(&mut buffered_input).unwrap().trim());
                output.write(b"Please, enter the surname: ").expect("");
                let surname = String::from(get_user_input(&mut buffered_input).unwrap().trim());
                output.write(b"Please, enter the age: ").expect("");
                let age = String::from(get_user_input(&mut buffered_input).unwrap().trim());

                let user = User { name, surname, age };

                let mut file = fs::OpenOptions::new()
                    .append(true)
                    .open(&file_path)
                    .unwrap();

                write!(&mut file, "{},{},{}\n", user.name, user.surname, user.age).expect("");
            }
            '3' => {
                let file_path = input_manager::get_filepath(&mut buffered_input, output);
                if file_path.is_none() {
                    continue;
                }

                let file_path = file_path.unwrap();

                let mut users = list_users(&file_path);

                for user in users.iter_mut() {
                    user.age = (user.age.parse::<i32>().unwrap() + 1).to_string();
                }

                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&file_path)
                    .unwrap();

                for user in users.iter() {
                    write!(&mut file, "{},{},{}\n", user.name, user.surname, user.age).expect("");
                }
            }
            '4' => (),
            'q' => break,
            _ => (),
        };
    }

    Ok(())
}

fn list_users(file_path: &Box<Path>) -> Vec<User> {
    let file = BufReader::new(fs::File::open(file_path).unwrap());

    let mut users: Vec<User> = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.split(",").collect();
        let user = User {
            name: String::from(v[0]),
            surname: String::from(v[1]),
            age: String::from(v[2]),
        };
        users.push(user);
    }

    users
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut input: &[u8] = b"1\n2\n3\n4\n5\n";
        let mut output: Vec<u8> = Vec::new();
        menu(&mut input, &mut output);
        let mut ff: Vec<u8> = Vec::new();
        let mut aa = Vec::from(&WELCOME_MSG[..]);
        let mut bb = Vec::from(&b"TODO1:\nTODO2:\nTODO3:\nTODO4:\n"[..]);
        ff.append(&mut aa);
        ff.append(&mut bb);
        assert_eq!(ff, output);
    }
}
