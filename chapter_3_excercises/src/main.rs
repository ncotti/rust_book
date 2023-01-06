use std::io;

fn main() {
    // Descomentar la función que se quiera ejecutar
    carol();

    fibonacci();

    celcius_to_farenheit();
}

fn celcius_to_farenheit() {
    println!("Celcius to Farenheit converter.");

    loop {
        println!("Enter temperature in Celcius:");

        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => continue,
        }

        let celcius: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        println!("Temperatura en Farenheit = {}", celcius * 9 / 5 + 32);
    }
}

fn fibonacci() {
    loop {
        let mut last: u32 = 0;
        let mut value: u32 = 1;
        let mut input: String = String::new();

        println!("Enter a fibonacci index.");

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from stdin!");

        let index: u32 = match input.trim().parse() {
            Ok(num) => {
                if num >= 1 {
                    num
                } else {
                    println!("Index must be > 1!");
                    continue;
                }
            }
            Err(_) => {
                println!("Not a number.");
                continue;
            }
        };

        if index == 1 {
            value = 0;
        } else if index == 2 {
            value = 1;
        } else {
            for _i in 3..index + 1 {
                let buffer: u32 = value;
                value += last;
                last = buffer;
            }
        }

        println!("Fibonacci value: {value}");
    }
}

fn carol() {
    const ORDINAL: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    const CAROL: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..ORDINAL.len() {
        println!("On the {} day of Christmas, my true love sent to me", ORDINAL[i]);
        for j in (0..i+1).rev() {
            if i != 1 && j == 0 {
                println!("And {}", CAROL[j].to_lowercase())
            } else {
                println!("{}", CAROL[j]);
            }
        }
        println!("");
    }

    println!("{}\n", CAROL[0]);
}
