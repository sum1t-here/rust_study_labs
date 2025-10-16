use std::fmt;

// A struct, or structure, is a custom data type that lets you package
// together and name multiple related values that make up a meaningful group.

// The #[derive(Debug)] attribute automatically generates
// code that allows this struct to be printed using {:?}
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// struct Greet {
//     greet: &str,
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are similar to functions: we declare them with the fn keyword and a name,
// they can have parameters and a return value, and they contain some code that’s
// run when the method is called from somewhere else.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.width
    }

    fn width(&self) -> bool {
        self.width > 0
    }
} // fn area can be removed now

// All functions defined within an impl block are called associated functions
// because they’re associated with the type named after the impl.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// The problem arises because &str is borrowed data, and Rust must ensure that the struct does not outlive the data it references.
// Ownership (String) avoids this issue entirely.

// missing lifetime specifier : complain from the compiler
// lifetime covered later

// To use `{}` (Display), we must implement fmt::Display manually
impl fmt::Display for User {
    // -------------------------------------------------------------
    // &self → a reference to the instance of User we’re trying to print
    // Example: if we print println!("{}", user1),
    //          then self = &user1 inside this function

    // f: &mut fmt::Formatter
    // -> 'f' is a mutable reference to the formatter that handles writing output
    // -> Think of it like a “printing context” that controls formatting details,
    //    such as width, alignment, etc.
    // -------------------------------------------------------------
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Define how you want it to be shown
        write!(f, "Hello, {}. Active status = {} ", self.username, self.active)
    }
}

// If you don’t implement the Display trait for your custom type (like a struct or enum),
// then using {} in println! will cause a compile-time error, not just print nothing.

pub fn run() {
    let user = User {
        active: true,
        username: String::from("anon"),
        email: String::from("anon@example.com"),
        sign_in_count: 1,
    };
    // Use {:?} to print debug info
    println!("{:?}", user);

    // User { active: true, username: "anon", email: "anon@example.com", sign_in_count: 1 }

    // Use {:#?} for pretty printing
    println!("{:#?}", user);

    // User {
    //     active: true,
    //     username: "anon",
    //     email: "anon@example.com",
    //     sign_in_count: 1,
    // };

    // dbg!() macro also prints the value along with line info
    dbg!(&user); // '&' avoids moving ownership

    // [src/books/the_rust_programming_language/chapters/ch5_structs/mod.rs:51:5] &user = User {
    //     active: true,
    //     username: "anon",
    //     email: "anon@example.com",
    //     sign_in_count: 1,
    // }

    println!("{}", user);
    // Hello, anon. Active status = true

    // Creating Instances from Other Instances with Struct Update Syntax

    let user1 = User {
        active: user.active,
        username: user.username,
        email: String::from("another@example.com"),
        sign_in_count: user.sign_in_count,
    };

    println!("{:?}", user1);
    // User { active: true, username: "anon", email: "another@example.com", sign_in_count: 1 }

    // The syntax .. specifies that the remaining fields not explicitly
    // set should have the same value as the fields in the given instance.

    let user2 = User {
        email: String::from("another@test.com"),
        ..user1
    };
    println!("{:?}", user2);
    // User { active: true, username: "anon", email: "another@test.com", sign_in_count: 1 }

    // Using Tuple Structs Without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black); // Color(0, 0, 0)
    println!("{:?}", origin); // Point(0, 0, 0)

    // initial version of rect

    // let width = 30;
    // let height = 20;

    // refactoring it with tuple
    // let rect = (30, 20);

    // refactoring using structs

    let rect = Rectangle {
        width: 30,
        height: 20,
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }

    let square = Rectangle::square(10);

    // println!("The area of the rectangle is {} square pixels.", area(&rect));
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
// Use &Rectangle when:
// You don’t want the function to take ownership.
// You want to avoid copying large structs.
// You only need read access.
