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
}
