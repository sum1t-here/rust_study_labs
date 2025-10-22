pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // failing test

    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };

    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };

    //     assert!(!smaller.can_hold(&larger));
    // }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Rust");
        assert!(result.contains("Rust"), "Greeting did not contain name, value was `{result}`");
    }

    // checking for panic
    #[test]
    // #[should_panic]
    #[should_panic(expected = "World")]
    fn greeting_contains_name_panic() {
        let result = greeting("World");
        assert!(result.contains("Rust"), "Greeting did not contain name, value was `{result}`");
    }

    // using Result<T, E> in Tests
    #[test]
    fn it_work() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }

        // #[test]
        // #[ignore]
        // fn expensive_test() {
        //     // code that takes an hour to run
        // }
    }
}

// $ cargo test -- --show-output
