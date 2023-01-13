use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let file = match File::open("hello.txt") {
        Ok(fd) => fd,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fd) => fd,
                Err(e) => panic!("Problem creating the file {}", e),
            },
            other_error => {
                panic!("Problem opening the file {}", other_error);
            },
        },
            
    };

    panic!("Panic Attack!!!!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    
    let mut file = match File::open("hello.txt") {
        Ok(fd) => fd,
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// El signo de pregunta hace que la función retorne el valor dentro de
// Ok(x) en caso de éxito, o salga de la función retornando Err(x) en caso
// de error.
fn read_username_from_file_with_question_mark() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
