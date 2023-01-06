fn main() {
    let condition: bool = true;

    if condition {
        5;
    } else {
        6;
    }

    println!("Returned: {}", my_function(5) );
}

fn my_function(value: i32) -> i32 {
    value
}