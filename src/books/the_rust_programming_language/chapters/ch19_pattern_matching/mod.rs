// matching named variables

pub fn run1() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

// Multiple Patterns

pub fn run2() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// Matching Ranges of Values with ..=

pub fn run3() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

// Destructuring Structs

pub fn run4() {
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring Enums

    enum Message {
        Quit,
        Move {
            x: i32,
            y: i32,
        },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main2() {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
        }
    }

    // Destructuring Nested Structs and Enums

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move {
            x: i32,
            y: i32,
        },
        Write(String),
        ChangeColor(Color),
    }

    fn main3() {
        let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message2::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message2::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}");
            }
            _ => (),
        }
    }
    // Ignoring Values in a Pattern
    // An Entire Value with _

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }

    fn main4() {
        foo(3, 4);
    }

    // Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // An Unused Variable by Starting Its Name with _
    fn main5() {
        let _x = 5;
        let y = 10;
    }

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

    // Remaining Parts of a Value with ..

    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {x}"),
    }

    fn main6() {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }
}

// Extra Conditionals with Match Guards
// A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen.

pub fn run5() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    fn main() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// @ Bindings
// The at operator @ lets us create a variable that holds a value at the same time we’re testing that value for a pattern match.

pub fn run6() {
    enum Message {
        Hello {
            id: i32,
        },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => { println!("Found an id in range: {id}") }
        Message::Hello { id: 10..=12 } => { println!("Found an id in another range") }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

// This example will print Found an id in range: 5. By specifying id @ before the range 3..=7, we’re capturing whatever value matched the range in a variable named id while also testing that the value matched the range pattern.
