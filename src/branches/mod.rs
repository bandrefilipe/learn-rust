#[allow(dead_code)]
pub fn execute() {
    if_expressions();
    handling_multiple_conditions();
    if_in_let_statements();
}

fn if_expressions() {
    let number = 3;

    if number < 5 {
        println!("conditions was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero")
    }
}

fn handling_multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_let_statements() {
    let condition = true;
    let number = if condition { 1 } else { 0 };
    println!("The value of number is: {}", number);
}
