// Ownership is a set of rules that govern how a Rust program manages memory.

// In Rust, whether data lives on the stack or the heap affects performance and ownership.
// Stack: Stores values in a last in, first out order; all data must have a fixed, known size.
// Heap: Used for data with unknown or variable size; allocation involves finding free space and returning a pointer to it.
// Stack operations (push/pop) are faster because they follow a simple order, while heap allocation is slower due to bookkeeping.
// Accessing heap data is slower since it requires following pointers, unlike the stack where data is contiguous.
// When a function is called, parameters and local variables are pushed onto the stack and popped when the function ends.
// Ownership in Rust manages heap data — tracking usage, avoiding duplicates, and freeing memory automatically.

// Ownership Rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

pub fn run() {
    // Stack allocation: fixed-size data
    let x = 10;
    let y = 20;
    println!("Stack values: x = {}, y = {}", x, y);
    // Stack values: x = 10, y = 20

    // Heap allocation: variable-size data
    let s1 = String::from("Hello, Rust!");
    println!("Heap value: {}", s1); // Heap value: Hello, Rust!

    // Ownership moves: s1 is moved to s2
    let s2 = s1;
    // println!("{}", s1); // ❌ Error: s1 no longer owns the data

    println!("s2 owns the heap data now: {}", s2); // s2 owns the heap data now: Hello, Rust!

    // Borrowing (no ownership transfer)
    print_length(&s2); // ✅ Borrowing allowed
    // Length of string: 12

    let slice = &s2[0..2];
    // The slice is: He
    println!("The slice is: {}", slice);
    let slice = &s2[..2];
    // The slice is: He
    println!("The slice is: {}", slice);
}

fn print_length(s: &String) {
    println!("Length of string: {}", s.len());
}

// A String is made up of three parts:
// a pointer to the memory that holds the contents of the string,
// a length, and a capacity.
// This group of data is stored on the stack.

// When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
// We do not copy the data on the heap that the pointer refers to.

// The Rules of References
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear(); // error!
//     println!("the first word is: {word}");
// }

// we have an immutable reference to something, we cannot also take a mutable reference

// Because clear needs to truncate the String, it needs to get a mutable reference.
// The println! after the call to clear uses the reference in word,
// so the immutable reference must still be active at that point.
// Rust disallows the mutable reference in clear and the immutable
// reference in word from existing at the same time, and compilation fails.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn run3() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    println!("{}", word);
    let word = first_word(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);
    println!("{}", word);
    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}", word);
}
