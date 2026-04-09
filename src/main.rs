fn main() {
    println!("Hello, world!");

    // Variables
    // By default, variables are immutable in Rust
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6; // cannot assign twice to immutable variable
    // println!("The value of x is: {x}");

    // Adding mut to a variable makes it mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The new value of x is: {x}");

    // Constants
    // Constants are immutable by default and cannot be changed with the mut keyword
    // They may only be set to a constant expression and not the result of a value to be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours.");
}
