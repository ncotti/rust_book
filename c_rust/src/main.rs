fn main() {
    let mut v = vec![1,10,100,1000];

    let (v1, v2) = c_rust::split_at_mut(&mut v, 2);

    println!("V1: {:?}; V2: {:?}", v1, v2);
}
