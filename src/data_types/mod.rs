#[allow(dead_code)]
pub fn execute() {
    // ########## Scalar types:
    //  - integers
    //  - floating-point numbers
    //  - booleans
    //  - characters

    // Integers:
    //  - signed:
    let _signed_8bits: i8;
    let _signed_16bits: i16;
    let _signed_32bits: i32;
    let _signed_64bits: i64;
    let _signed_128bits: i128;
    let _signed_arch: isize;
    //  - unsigned:
    let _unsigned_8bits: u8;
    let _unsigned_16bits: u16;
    let _unsigned_32bits: u32;
    let _unsigned_64bits: u64;
    let _unsigned_128bits: u128;
    let _unsigned_arch: usize;
    //  e.g. number literals:
    let _decimal = 98_222; // 98222
    let _hex = 0xff; // 255
    let _octal = 0o77; // 63
    let _binary = 0b1111_0000; // 240
    let _byte = b'A'; // 65

    // Floating-Point Numbers:
    let _float: f32 = 2.0;
    let _double: f64 = 2.0; // default
    // numeric operations:
    //  - addition:
    let _sum = 5 + 10;
    //  - subtraction:
    let _difference = 95.5 - 4.3;
    //  - multiplication:
    let _product = 4 * 30;
    //  - division:
    let _quotient = 56.7 / 32.2;
    //  - remainder:
    let _remainder = 43 % 5;

    // Booleans:
    let _t: bool = true;
    let _f: bool = false;

    // Characters:
    let _c: char = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat: char = '😻';

    // ########## Compound types:
    //  - tuples (may be heterogeneous)
    //  - arrays (must be homogeneous)

    // Tuples:
    let tuple: (i32, f64, u8) = (500, 6.4, 1); // optionally typed
    // we can use pattern matching to destructure a tuple:
    let (x, y, z) = tuple;
    println!("The tuple's values: x={}, y={}, z={}", x, y, z);
    // we can also access a tuple value by its index:
    let five_hundred = tuple.0;
    println!("The tuple's value at index 0: {}", five_hundred);

    // Arrays:
    let array: [i32; 5] = [1, 2, 4, 8, 16];
    let _first = array[0];
    let _second = array[1];
    let _last = array[array.len() - 1];
}
