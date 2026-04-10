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

    // Shadowing
    // Shadowing allows a new variable to be declared with the same name as an existing variable
    // The new variable takes precedence and shadows the old one
    // You however cannot use the mut keyword with shadowed variables, if not, the compiler will raise an error
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y inside the block is: {y}");
    }
    println!("The value of y outside the block is: {y}");
    // It is also permitted for the value to be of a different type from the previous value
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {spaces}");
}
