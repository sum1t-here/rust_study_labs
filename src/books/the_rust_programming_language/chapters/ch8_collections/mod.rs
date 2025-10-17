use std::collections::HashMap;

pub fn run() {
    // creating a vector
    // let v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];

    // updating a vector
    v.push(5);

    // reading elements from a vector
    let third = &v[2];
    println!("The third element is {third}");
    // The third element is 3

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
    // The third element is 3

    // string
    // let s = String::from("Hello");
    // let s = String::new();

    let data = "Initial commits";
    let mut s = data.to_string();

    // updating a string
    s.push_str("world");

    println!("{s}");
    // Initial commitsworld

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s1_clone = s1.clone();
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s4 = format!("{s1_clone}-{s2}-{s3}");
    println!("{s4}");
    // Hello, -world!-Hello, world!

    // Indexing into Strings
    let s5 = String::from("hi");
    let h = s5.chars().nth(0).unwrap();
    println!("{h}"); // h

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}"); // Зд

    // iterating over a string
    for c in "Зд".chars() {
        println!("{c}");
    }
    // З
    // д

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // 208
    // 151
    // 208
    // 180

    // creating a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    // accessing values in hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}"); // 10

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Yellow: 20
    // Blue: 10

    // overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");
    // {"Yellow": 20, "Blue": 25}

    // adding a Key and Value Only If a Key Isn’t Present
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");
    // {"Yellow": 20, "Blue": 25}

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // * → dereference (access the value inside a reference)
        *count += 1;
    }
    println!("{map:?}");
    // {"hello": 1, "world": 2, "wonderful": 1}
}
