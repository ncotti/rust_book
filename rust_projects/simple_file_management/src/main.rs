use simple_file_management::menu;
use std::io;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    menu(&mut stdin, &mut stdout).expect("Exito");
}
