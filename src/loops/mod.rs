#[allow(dead_code)]
pub fn execute() {
    returning_values_from_loops();
    conditional_loops_with_while();
    looping_through_a_collection_with_for();
    looping_with_range();
}

fn returning_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // this expression is returned
        }
    };

    println!("The result is: {}", result);
}

fn conditional_loops_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_through_a_collection_with_for() {
    let array = [10, 20, 30, 40, 50];
    // let mut index = 0;
    //
    // while index < array.len() {
    //     println!("The value is: {}", array[index]);
    //     index += 1;
    // }
    for element in array.iter() {
        println!("The value is: {}", element);
    }
}

fn looping_with_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
