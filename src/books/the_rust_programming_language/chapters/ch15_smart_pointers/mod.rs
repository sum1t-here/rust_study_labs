// A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data. The most common kind of pointer in Rust is a reference,

// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities

// Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers. The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

// We’ll cover the most common smart pointers in the standard library:

// Box<T>, for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

// Using Box<T> to Point to Data on the Heap
// Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

// You’ll use them most often in these situations:
// When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
// When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
// When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

use std::{ cell::RefCell, ops::Deref, rc::{ Rc, Weak } };

// Using Box<T> to Store Data on the Heap
pub fn box1() {
    let b = Box::new(5);
    println!("b = {b}")
}
// b = 5

// Enabling Recursive Types with Boxes
// A value of a recursive type can have another value of the same type as part of itself. Recursive types pose an issue because Rust needs to know at compile time how much space a type takes up. However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

// A cons list is a data structure that comes from the Lisp programming language and its dialects, is made up of nested pairs, and is the Lisp version of a linked list.

// Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called Nil without a next item. A cons list is produced by recursively calling the cons function

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// recursive type `List` has infinite sizerustcClick for full compiler diagnostic
// mod.rs(35, 15): recursive without indirection
// mod.rs(35, 15): insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle: `Box<`, `>`

pub fn box2() {
    use List::{ Cons, Nil };

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Treating Smart Pointers Like Regular References with Deref
// Implementing the Deref trait allows you to customize the behavior of the dereference operator *

// Let’s first look at how the dereference operator works with regular references. Then we’ll try to define a custom type that behaves like Box<T>, and see why the dereference operator doesn’t work like a reference on our newly defined type. We’ll explore how implementing the Deref trait makes it possible for smart pointers to work in ways similar to references. Then we’ll look at Rust’s deref coercion feature and how it lets us work with either references or smart pointers.

// Following the Reference to the Value

pub fn box3() {
    let x = 5;
    // let y = &x;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Defining Our Own Smart Pointer

// Let’s build a wrapper type similar to the Box<T> type provided by the standard library to experience how smart pointer types behave differently from references by default. Then we’ll look at how to add the ability to use the dereference operator.

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // The type Target = T; syntax defines an associated type for the Deref trait to use. Associated types are a slightly different way of declaring a generic parameter
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn box4() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // When we entered *y behind the scenes Rust actually ran this code:
    // *(y.deref())
}

// Implicit Deref Coercions with Functions and Methods
// Deref coercion is a Rust feature that automatically converts references of one type into references of another type using the Deref trait.

// It helps you write cleaner and more intuitive code when working with smart pointers (like Box, Rc, or custom wrappers).

pub fn box5() {
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    fn main() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m); // Works due to deref coercion
    }
}

// What Happens:

// &m → &MyBox<String>
// Rust sees MyBox<T> implements Deref<Target = T>
// So it calls deref() → gets &String
// Then automatically calls another deref() from String → &str
// Matches hello(&str) perfectly.

// Without Deref Coercion
// You’d need to manually write:

// hello(&(*m)[..]);

// Explanation:
// *m → dereferences MyBox<String> → gives a String
// [..] → creates a slice of the whole string
// & → gives a &str

// How Deref Coercion Interacts with Mutability
// Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

// Rust does deref coercion when it finds types and trait implementations in three cases:

// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

// Running Code on Cleanup with the Drop Trait
// The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn box6() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!

pub fn box7() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // c.drop(); explicit destructor calls not allowed
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
// CustomSmartPointer created.
// Dropping CustomSmartPointer with data `some data`!
// CustomSmartPointer dropped before the end of main.

// Rc<T>, the Reference Counted Smart Pointer
// The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

// Using Rc<T> to Share Data
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

pub fn box8() {
    use List2::{ Cons, Nil };

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

// Cloning an Rc<T> Increases the Reference Count

pub fn box9() {
    use List2::{ Cons, Nil };

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2

// RefCell<T> and the Interior Mutability Pattern
// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.

// RefCell<T> type represents single ownership over the data it holds
// At any given time, you can have either one mutable reference or any number of immutable references (but not both).
// References must always be valid.
// RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
// Mutating the value inside an immutable value is the interior mutability pattern.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = (self.value as f64) / (self.max as f64);

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
            // `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Reference Cycles Can Leak Memory
// Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak). Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust. We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

#[derive(Debug)]
enum List3 {
    Cons(i32, RefCell<Rc<List3>>),
    Nil,
}

use List3::{ Cons, Nil };

impl List3 {
    fn tail(&self) -> Option<&RefCell<Rc<List3>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn box10() {
    // 🧩 Create a linked list node "a" that holds:
    // value = 5, next = Nil (empty)
    // Rc = enables multiple ownership
    // RefCell = enables interior mutability (we can modify through Rc)
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); // → 1
    println!("a next item = {:?}", a.tail());

    // 🧩 Create another node "b" that points to "a" as its next node
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // At this point:
    // a -> Nil
    // b -> a
    // Rc strong counts:
    // a: 2 (one for 'a', one inside 'b')
    // b: 1 (only in variable 'b')

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // → 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // → 1
    println!("b next item = {:?}", b.tail());

    // 🧨 Now we create a *cycle*:
    // a.tail() = RefCell containing Rc<List> → currently points to Nil
    // We mutate it to point to b instead
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // Now:
    // a -> b
    // b -> a
    // 🚫 Reference cycle formed!

    // Rc counts increase again:
    // a: 2 (b points to a)
    // b: 2 (a points to b)
    // Neither count can ever drop to 0 → memory leak

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // → 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // → 2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());

    // ❌ Uncommenting the below line will cause infinite recursion
    // because Rust will keep traversing a → b → a → b → ...
}

// a initial rc count = 1
// a next item = Some(RefCell { value: Nil })
// a rc count after b creation = 2
// b initial rc count = 1
// b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
// b rc count after changing a = 2
// a rc count after changing a = 2

// 🧠 Rc<T> + Weak<T> — Preventing Reference Cycles
//
// - Rc<T>::clone() → increases *strong_count* (shared ownership).
// - Rc<T> value is dropped only when *strong_count == 0*.
//
// 🔗 Problem:
//   Cycles can form if two Rc<T> values reference each other → memory leak!
//   (strong_count never reaches 0)
//
// ✅ Solution → use Weak<T> via Rc::downgrade(&rc):
//   - Creates a *Weak<T>* reference (no ownership).
//   - Increases *weak_count*, not *strong_count*.
//   - *weak_count* doesn’t prevent cleanup.
//
// ⚙️ Accessing Weak<T> data:
//   - Use weak_ref.upgrade()
//   - Returns Option<Rc<T>>:
//       → Some(Rc<T>) if value still exists
//       → None if Rc<T> already dropped
//
// 🔒 Benefit:
//   Weak<T> breaks potential reference cycles
//   (common in tree or graph structures — e.g., parent ↔ child links)
//
// 💡 Example Use:
//   - Child holds Weak<T> to parent
//   - Parent holds Rc<T> to children

// Creating a Tree Data Structure: A Node with Child Nodes
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn box11() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}

// Adding a Reference from a Child to Its Parent
#[derive(Debug)]
struct Node2 {
    value: i32,
    parent: RefCell<Weak<Node2>>,
    children: RefCell<Vec<Rc<Node2>>>,
}

pub fn box12() {
    let leaf = Rc::new(Node2 {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node2 {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// leaf parent = None
// leaf parent = Some(Node2 { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node2 { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
