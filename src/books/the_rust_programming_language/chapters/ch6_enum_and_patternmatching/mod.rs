use std::fmt::format;

#[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// format of Option
// enum Option<T> {
//     None,
//     Some(T),
// }

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn run() {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{:?}", home);

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", loopback);

    // In Rust, `Option<T>` is used when a value can be **present or absent**.
    // It is an enum with two variants:
    // - Some(value) → The value exists
    // - None → The value is absent
    // Using Option helps avoid null pointer errors and makes code safer.

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    fn find_user(user_id: u32) -> Option<String> {
        if user_id == 1 {
            // Found the user, return Some(value)
            Some(String::from("Alice"))
        } else {
            // User not found, return None
            None
        }
    }

    // Call the function
    let user = find_user(1);

    // Handle the Option using match
    match user {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }

    // Another way: use unwrap_or to provide a default value
    let user_name = find_user(2).unwrap_or(String::from("Guest"));
    println!("User: {}", user_name);

    // Option is safe replacement for null.
    // Forces the programmer to handle the case when value is absent.
    // Can be handled with match, if let, unwrap_or, etc.
    // Makes your code more robust and error-resistant.
    let coin = Coin::Penny;
    println!("{:?}", value_in_cents(&coin)); // 1

    // if let is syntactic sugar for a match when you only care about one specific pattern and want to ignore the rest.
    // It reduces boilerplate because you don’t need _ => () to ignore other cases.

    let config_max = Some(3u8);

    // Using match (verbose)
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (), // required boilerplate
    }

    // Using if let (concise)
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin = Coin::Quarter;
    if let Coin::Quarter = coin {
        println!("Quarter");
    } else {
        count += 1; // all other coins
    }
    println!("{count}");
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn describe_cents(coin: Coin) -> Option<String> {
    let Coin::Quarter = coin else {
        return None; // exit early if pattern does not match
    };

    // Example: check if coin is Penny using match
    // (But since we already matched Quarter above, this will never be Penny)
    // For demonstration, let's just return Some for Quarter
    Some(format!("Quarter it is"))

    // // First, check if the coin is a Quarter
    // if let Coin::Quarter = coin {
    //     // Nested check: for demonstration, let's pretend we want to see if
    //     // some additional condition is true; here we'll just simulate it
    //     let year = 1900;
    //     if year < 2000 {
    //         Some(String::from("Quarter from before 2000"))
    //     } else {
    //         Some(String::from("Quarter from after 2000"))
    //     }
    // } else {
    //     // If the coin is not a Quarter, return None
    //     None
    // }
}
