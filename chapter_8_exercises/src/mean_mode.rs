use std::collections::HashMap;

pub fn mean_mode_main(vec: &[u32]) {
    let mut input: Vec<u32> = Vec::from(vec);

    println!("Median: {}", get_median(&mut input));

    println!("Mode: ");
    for element in get_mode(&input) {
        println!("Number:{}, Times:{}", element.0, element.1);
    }
}


fn get_mode(vec: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut output_vec: Vec<(u32, u32)> = Vec::new();

    for element in vec {
        match map.get_mut(&element) {
            Some(value) => {*value += 1;},
            None => {match map.insert(*element, 1) {
                    Some(_) => (),
                    None => (),
                }
            },
        }
    }

    let mut max: u32 = 0;

    for value in map.values() {
        if *value > max {
            max = *value;
        }
    }

    for element in map {
        if max == element.1 {
            output_vec.push(element);
        }
    }

    output_vec
}

fn get_median(vec: &mut Vec<u32>) -> f64 {
    vec.sort();

    if vec.len() % 2 == 0 {
        (vec[vec.len()/2] + vec[vec.len()/2 - 1]) as f64 / 2 as f64
    } else {
        vec[vec.len()/2] as f64
    }
}