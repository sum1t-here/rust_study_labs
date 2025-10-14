pub fn run() {
    // By default, variables are immutable
    // let x = 5; // will run into error
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // since x is changing here we will run into error if above x is not set to mut
    println!("The value of x is: {x}");

    // Constants
    const THREE: u32 = 3;
    println!("The value of constant is: {THREE}");

    // Shadowing

    // Shadowing in Rust means redeclaring a variable with the same name,
    // allowing you to change its value or even its type while keeping
    // the same variable name in scope.

    let x = x + 1; // 7

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 14
    }

    println!("The value of x is: {x}"); // 7
}
