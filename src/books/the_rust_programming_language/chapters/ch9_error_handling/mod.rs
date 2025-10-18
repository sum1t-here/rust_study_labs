use std::{ fs::File, io::{ self, Read } };

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => {
    //         return Err(e);
    //     }
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // ? -> shortcut to propagate error

    // The ? placed after a Result value is defined to work in almost
    // the same way as the match expressions we defined to handle the
    // Result values
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn run() {
    // unrecoverable errors with panic!
    // panic!("crash and burn");

    // Result<T, E>
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };

    // unwrap and expect

    // If the Result value is the Ok variant,
    // unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the
    // panic! macro for us.

    // let greeting_file_result = File::open("hello.txt").unwrap();

    // let greeting_file = File::open("hello.txt").expect(
    //     "hello.txt should be included in this project"
    // );

    read_username_from_file();
}
