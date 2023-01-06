fn main() {
    while_loop();
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn nested_loops() {
    let mut count = 0;
    'label: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'label;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}
