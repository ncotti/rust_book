//! This module contains all the messages displayed on Stdout.

pub const WELCOME_MSG: &[u8; 129] = b"Welcome to our user database.\n\
Please, choose an option:\n\
1. List users.\n\
2. Add new user.\n\
3. Modify user.\n\
4. Delete user.\n\
q. Exit.\n";

pub const ASKING_FOR_USER_FILE: &[u8; 33] = b"Enter the path to the user file: ";
