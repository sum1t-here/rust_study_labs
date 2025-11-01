// Closures, a function-like construct you can store in a variable
// Iterators, a way of processing a series of elements

// Closures:

use std::{ thread, time::Duration };

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // This function decides which shirt to give away to the user.
    // If the user has a preferred color, give that one.
    // If not, use a closure to determine which color is most stocked.
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // `unwrap_or_else` takes a closure: if `user_preference` is None,
        // it calls the closure and returns its result.
        user_preference.unwrap_or_else(|| self.most_stocked())
        // Note: `|| self.most_stocked()` is a closure (no arguments) that returns a ShirtColor.
        // It is only called if `user_preference` is None.
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => {
                    num_red += 1;
                }
                ShirtColor::Blue => {
                    num_blue += 1;
                }
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn run() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);

    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    // closure type inference and annotation
    // Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
    // Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let result = expensive_closure(5);
    println!("Result: {}", result); // Result: 5
}

// The user with preference Some(Red) gets Red
// The user with preference None gets Blue

// capturing ref or moving ownership
pub fn run2() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // Closure that just borrows `list`
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows(); // closure called here
    println!("After calling closure: {list:?}");
}

// Before defining closure: [1, 2, 3]
// Before calling closure: [1, 2, 3]
// From closure: [1, 2, 3]
// After calling closure: [1, 2, 3]

pub fn run3() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

// Before defining closure: [1, 2, 3]
// After calling closure: [1, 2, 3, 7]

// If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the move keyword before the parameter list.

pub fn run4() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // Spawn a new thread to run the closure
    // `move` forces the closure to take ownership of variables it uses
    // In this case, it transfers ownership of `list` into the new thread

    // This closure now owns `list`
    // So it can safely access it inside the new thread
    thread
        ::spawn(move || println!("From thread: {list:?}"))
        // Wait for the spawned thread to finish before continuing
        // If the thread panics, `.unwrap()` will propagate the panic
        .join()
        .unwrap();

    // ⚠️ We can’t use `list` here anymore — ownership was moved into the new thread
    // println!("{list:?}");  // ❌ This would cause a compile error
}

// Moving Captured Values Out of Closures and the Fn Traits

/*
🧠 RUST CLOSURE TRAITS — SHORT NOTES

Closures can capture values from their environment in different ways:
1. Move captured value out of the closure
2. Mutate captured value
3. Borrow immutably (neither move nor mutate)
4. Capture nothing

Depending on how they capture and use values, closures automatically
implement one or more of these traits:

──────────────────────────────────────────────
FnOnce → moves captured values
  • Can be called only once
  • Consumes ownership of captured variables

FnMut → mutates captured values
  • Can be called multiple times
  • Requires &mut access to environment

Fn → borrows captured values immutably
  • Can be called many times safely
  • Thread-safe, doesn’t mutate or consume
──────────────────────────────────────────────

🪄 Key Rule:
• Every closure implements FnOnce.
• If it doesn’t move values, it also implements FnMut.
• If it neither moves nor mutates, it also implements Fn.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
pub fn run5() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    /*
    💡 list.sort_by_key(|r| r.width);

    - The method `sort_by_key` expects a closure that tells it how to extract
      a sorting "key" from each element (here, the width of the rectangle).

    - The closure `|r| r.width` takes an immutable reference to each element (`r`)
      and simply returns the `width` field.

    🧠 Closure Trait Context:
    ---------------------------------
    - This closure only **reads** data from its environment.
    - It **does not mutate** anything or **take ownership** of `r`.
    - Therefore, it implements the **Fn** trait.
      (Closures that only borrow immutably implement Fn.)

    🔍 What happens here:
    - Rust passes each rectangle reference to the closure.
    - The closure returns the width of that rectangle.
    - The sort function uses those widths to sort the array in ascending order.

    After sorting, rectangles will be ordered by width:
      width: 3 → 7 → 10
    */

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

// [
//     Rectangle {
//         width: 3,
//         height: 5,
//     },
//     Rectangle {
//         width: 7,
//         height: 12,
//     },
//     Rectangle {
//         width: 10,
//         height: 1,
//     },
// ];

pub fn run6() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // This variable lives in the outer scope of the closure
    // We’ll mutate it inside the closure to count how many times the closure is called
    let mut num_sort_operations = 0;

    // This string is never actually used inside the closure, but
    // shows that closures can also capture values from the environment
    let value = String::from("closure called");

    /*
    💡 list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    - `sort_by_key` sorts elements based on the "key" returned by the closure.
    - The closure takes an immutable reference `r` to each Rectangle in the list.
    - It returns `r.width` as the sorting key.
    - Before returning, it increments `num_sort_operations` each time it’s called.

    ⚙️ What happens:
    - The sort algorithm calls the closure multiple times to compare elements.
    - Each call increases `num_sort_operations` by 1.
    - The rectangles get sorted by width in ascending order (3, 7, 10).

    🧠 Closure Trait Context:
    - The closure *mutates* a variable (`num_sort_operations`) from its outer scope.
    - That means it requires a **mutable borrow (&mut)** of that variable.
    - So, this closure implements **FnMut**.
      → Can be called multiple times, but needs mutable access.

    If it only *read* the variable, it would implement **Fn**.
    If it *moved* ownership (like consuming `value`), it would implement **FnOnce**.
    */

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}");
}

// [
//     Rectangle {
//         width: 3,
//         height: 5,
//     },
//     Rectangle {
//         width: 7,
//         height: 12,
//     },
//     Rectangle {
//         width: 10,
//         height: 1,
//     },
// ]

// Processing a Series of Items with Iterators
pub fn iterator() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

// The Iterator Trait and the next Method
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
// Methods That Consume the Iterator

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Methods That Produce Other Iterators
pub fn iterator2() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1
        .iter()
        .map(|x| x + 1)
        .collect();

    println!("{v2:?}")
}
// [2, 3, 4]

// Using Closures That Capture Their Environment
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            }
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                }
            ]
        );
    }
}
