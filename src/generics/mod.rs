#[allow(dead_code)]
pub fn execute() {
    in_function_definitions();
    in_struct_definitions();
    in_enum_definitions();
    in_method_definitions();
}

fn in_function_definitions() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    assert_eq!(result, &100);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    assert_eq!(result, &'y');

    let string_list = vec![
        "test".to_string(),
        "zebra".to_string(),
        "helicopter".to_string(),
    ];
    let result = largest(&string_list);
    assert_eq!(result, &"zebra");
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn in_struct_definitions() {
    let point_i32: Point<i32> = Point { x: 5, y: 10 };
    let point_f64: Point<f64> = Point { x: 1.0, y: 4.0 };

    println!("point_i32 = {:?}", point_i32);
    println!("point_f64 = {:?}", point_f64);
}

fn in_enum_definitions() {
    #[derive(Debug)]
    enum Optional<T> {
        Value(T),
        Empty,
    }
    let opt: Optional<i32> = Optional::Value(42);
    println!("opt: {:?}", opt);
    let opt: Optional<String> = Optional::Empty;
    println!("opt: {:?}", opt);
}

fn in_method_definitions() {
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    impl Point<f32> {
        //Only applies to a struct with a particular concrete type for the generic type parameter T.
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let point_f32: Point<f32> = Point { x: 2.0, y: 4.5 };
    println!("distance: {}", point_f32.distance_from_origin());

    mixup_example();
}

fn mixup_example() {
    #[derive(Debug)]
    struct Point<X, Y> {
        x: X,
        y: Y,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1: Point<i32, u32> = Point { x: -1, y: 1 };
    let p2: Point<&str, bool> = Point { x: "x", y: true };
    let p3: Point<i32, bool> = p1.mixup(p2);
    println!("p3 = {:?}", p3);
}
