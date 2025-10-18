// The 'T: PartialOrd' means that the generic type T must implement the 'PartialOrd' trait,
// which allows comparison operators like '>' and '<' to be used on values of type T.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// A trait defines the functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// they are similar to interface
pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// implementing a Trait on a type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// default implementations
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// trait as parameter
pub fn notify(item: &impl Summary) {
    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// above one is syntatic sugar for the below one (trait bounds)
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
// multiple traits
// pub fn notify<T: Summary + Display>(item: &T) {
// pub fn notify(item: &(impl Summary + Display)) {

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Lifetime Annotations in Method Definitions
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// The Static Lifetime
// One special lifetime we need to discuss is 'static,
// which denotes that the affected reference can live
// for the entire duration of the programecial lifetime
// we need to discuss is 'static, which denotes that the
// affected reference can live for the entire duration of the program

// let s: &'static str = "I have a static lifetime.";

use std::fmt::Display;
// Generic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."
        ),
    };

    println!("New article available! {}", article.summarize());
    notify(&post);

    // lifetime

    // ❌ Example showing a lifetime error
    // let r;                  // `r` is declared here but not yet assigned — its lifetime starts
    // {
    //     let x = 5;          // `x` is created inside this inner block — its lifetime starts here
    //     r = &x;             // ❌ ERROR: `r` borrows `x`, but `x` will be dropped at the end of this block
    // }                       // `x` goes out of scope — `r` now points to invalid memory
    // println!("r: {r}");     // would cause a dangling reference if allowed

    // ✅ Fixed version — same lifetime scope
    let x = 5; // `x` is created here
    let r = &x; // `r` borrows `x`; both live in the same (outer) scope
    println!("r: {r}"); // valid — `x` is still alive when used through `r`

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
