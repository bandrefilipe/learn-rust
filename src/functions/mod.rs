#[allow(dead_code)]
pub fn execute() {
    println!("Some function");
    another_function(5, 6);

    // statements & expressions:
    let _x = 5;
    let _y = {
        let x = 3;
        x + 1 // no ;
    }; // _y == 4
    let _five = five();
    let _six = plus_one(five());
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(n: i32) -> i32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn four_plus_one_equals_five() {
        assert_eq!(plus_one(4), 5);
    }
}
