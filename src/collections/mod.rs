#[allow(dead_code)]
pub fn execute() {
    storing_lists_of_values_with_vectors();
    storing_utf8_encoded_text_with_strings();
}

fn storing_lists_of_values_with_vectors() {
    creating_a_new_vector();
    updating_a_vector();
    dropping_a_vector_drops_its_elements();
    reading_elements_of_vectors();
    iterating_over_the_values_in_a_vector();
    using_an_enum_to_store_multiple_types();
}

fn storing_utf8_encoded_text_with_strings() {
    todo!();
}

fn creating_a_new_vector() {
    // Creating a new, empty vector to hold values of type i32
    let _v: Vec<i32> = Vec::new();
    // Creating a new vector containing values
    let _v = vec![1, 2, 3];
}

fn updating_a_vector() {
    let mut v = Vec::new();
    // Using the `push` method to add values to a vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

/// Showing where the vector and its elements are dropped.
fn dropping_a_vector_drops_its_elements() {
    let _v = vec![1, 2, 3];
    // do stuff with _v
} // <- goes out of scope and is freed here

/// Shows both methods of accessing a value in a vector, either with indexing syntax or the `get` method.
fn reading_elements_of_vectors() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let _first = &v[0];
    v.push(6);
    // println!("The first element is {}", _first); // <- won't compile if uncommented
}

fn iterating_over_the_values_in_a_vector() {
    // Printing each element in a vector by iterating over the elements using a for loop
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // Iterating over mutable references in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // the dereference operator (*) is used to get the value in i
        *i += 50;
    }
}

/// Defining an `enum` to store values of different types in one vector.
fn using_an_enum_to_store_multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in &row {
        println!("{:?}", cell);
    }
}
