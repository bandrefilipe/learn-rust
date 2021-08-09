#[allow(dead_code)]
pub fn execute() {
    let x = 5; // `x` is immutable by default
    println!("The value of x is: {}", x);
    // compile-time error: cannot assign twice to immutable variable `x`
    // x = 6;

    let mut y = 5; // `y` is mutable
    println!("The value of y is: {}", y);
    y = 6; // compiler allows it
    println!("The new value of y is: {}", y);

    // constants must always be annotated with type and can be declared in any scope
    const MAX_POINTS: u32 = 100_000;

    // shadowing:
    let z = 5;
    let z = z + 1; // creates a new `z` variable and shadows the above one
    println!("The value of z is: {}", z);
    // another shadowing example:
    let spaces: String = String::from("     ");
    let spaces: usize = spaces.len(); // we can even change types!
    println!("The size of spaces is: {}", spaces);
}
