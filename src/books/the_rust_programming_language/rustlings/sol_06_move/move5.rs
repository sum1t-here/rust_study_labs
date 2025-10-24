#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    // Convert the string into an iterator over its characters
    // and get the last character (if any) using `.last()`.
    // `.unwrap()` is used to extract the character from the Option,
    // which will panic if the string is empty.
    let char = data.chars().last().unwrap();

    // Print the last character to the console
    println!("{char}");
    char
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

pub fn run() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
