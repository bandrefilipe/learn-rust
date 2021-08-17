#[allow(dead_code)]
pub fn execute() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Debug: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
