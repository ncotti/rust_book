#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let _six = IpAddrKind::V6(String::from("::1"));

    let x = 5;
    let y = Some(6);
    let z: Option<u32> = None;

    let config_max = Some::<u8>(3);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    println!("{}", x + y.unwrap()); // This may panick if "y" is None, match expression is preferred

    match z {
        None => println!("None"),
        Some(num) => println!("{}", num),
    }

    match y {
        None => println!("None"),
        Some(num) => println!("{}", num),
    }

    println!("Coin: {}", value_in_cents(Coin::Nickel));

    ip_sort(four);

}

fn ip_sort(ip: IpAddrKind) {
    match ip {
        IpAddrKind::V4(a, b, c, d) => println!("{}.{}.{}.{}",a,b,c,d),
        IpAddrKind::V6(name) => println!("{}", name),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
