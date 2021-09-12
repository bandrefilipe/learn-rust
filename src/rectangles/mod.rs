#[allow(dead_code)]
pub fn execute() {
    defining_methods();
    methods_with_more_parameters();
    associated_functions();
}

fn defining_methods() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Debug: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

fn methods_with_more_parameters() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn associated_functions() {
    let sq = Rectangle::square(3);
    println!("Debug: sq={:?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 7,
            height: 6,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 7,
            height: 6,
        };
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        assert!(!smaller.can_hold(&larger));
    }
}