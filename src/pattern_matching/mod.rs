#[allow(dead_code)]
pub fn execute() {
    patterns_that_bind_to_values();
    matching_with_option();
}

fn patterns_that_bind_to_values() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alabama));
}

fn matching_with_option() {
    let _five = Some(5);
    let _six = plus_one(_five);
    let _none = plus_one(None);

    some_u8_value(0);
    some_u8_value(1);
    some_u8_value(7);
    some_u8_value(255);
}

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn some_u8_value(x: u8) {
    match x {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}