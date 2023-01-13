use std::collections::HashMap;
use std::io;

pub fn company_main() {
    let mut departments: HashMap<&str, Vec<String>> = HashMap::new();
    departments.insert("HHRR", Vec::new());
    departments.insert("Accountancy", Vec::new());
    departments.insert("Psicology", Vec::new());

    loop {
        let mut text_input: String = String::new();

        println!("\nWelcome to our company. What do you wish?");
        println!("1. Add new member to HHRR.");
        println!("2. Add new member to Accountancy.");
        println!("3. Add new member to Psicology.");
        println!("4. Print all members in all departments.");
        println!("5. Exit.");

        match io::stdin().read_line(&mut text_input) {
            Ok(_) => (),
            Err(_) => {
                println!("Error while reading stdin.");
                continue;
            }
        }

        let dep: &str = match text_input.chars().next().unwrap() {
            '1' => "HHRR",
            '2' => "Accountancy",
            '3' => "Psicology",
            '4' => {
                print_departments(&departments);
                continue;
            },
            '5' => break,
            _ => {
                println!("Invalid argument.");
                continue;
            }
        };


        println!("Enter the name of your new colleague");

        text_input.clear();
        match io::stdin().read_line(&mut text_input) {
            Ok(_) => (),
            Err(_) => {
                println!("Error while reading stdin.");
                continue;
            }
        }
        let vec = departments.get_mut(dep).unwrap();
        vec.push(String::from(text_input.trim()));
    }
}

fn print_departments(departments: &HashMap<&str, Vec<String>>) {
    for (dep, person_vec) in departments.iter() {
        println!("{}", *dep);
        for person in person_vec {
            println!("    {}", *person);
        }
    }
}