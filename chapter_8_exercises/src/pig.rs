use regex::Regex;

pub fn pig_main(input: &str) {
    let mut output: String = String::new();
    let vocal = Regex::new(r"(?i)^[aeiou]").unwrap();
    let re = Regex::new(r"(?i)^[a-z]+$").unwrap();
    
    if re.is_match(input) {
        if re.is_match(input) {
            if vocal.is_match(input) {
                output.push_str(input);
                output.push_str("-hay");
                
            }

            else {
                output.push_str(input);
                output.push('-');
                let first = output.remove(0);
                output.push(first);
                output.push_str("ay");
            }

            println!("{}", &output);
    }

    } else {
        println!("Wrong word!");
    }
    
}