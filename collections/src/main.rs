fn main() {
    let mut v: Vec<u32> = Vec::from([1,2,3,4,5]);

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

}
