#[allow(dead_code)]
pub fn execute() {
    creating_a_new_string();
    updating_a_string();
    slicing_strings();
    methods_for_iterating_over_strings();
}

fn creating_a_new_string() {
    let _s: String = String::new();
    let data: &str = "initial contents";
    let _s: String = data.to_string();
    let _s: String = String::from("initial contents");

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
}

fn updating_a_string() {
    appending_to_a_string_with_push_str_and_push();
    concatenation_with_the_plus_operator_or_the_format_macro();
}

fn appending_to_a_string_with_push_str_and_push() {
    let mut s: String = String::from("foo");
    let bar: &str = "bar";
    s.push_str(bar);
    println!("bar is {}", bar); // bar was borrowed, so it still can be used.

    let mut s: String = String::from("lo");
    s.push('l');
    assert_eq!(s, "lol");
}

fn concatenation_with_the_plus_operator_or_the_format_macro() {
    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2; // notice s1 has been moved here and can no longer be used
    assert_eq!(s3, "Hello, world!");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = tic + "-" + &tac + "-" + &toe;
    assert_eq!(tic_tac_toe, "tic-tac-toe");

    let tic = String::from("tic");
    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe); // tic is borrowed, so we could still use it
    assert_eq!(tic_tac_toe, "tic-tac-toe");
}

fn slicing_strings() {
    let hello = String::from("Здравствуйте");
    let s = &hello[..4];
    assert_eq!(s, "Зд");

    // let _it_panics = &hello[..3];
    // panicked at 'byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`'
}

fn methods_for_iterating_over_strings() {
    // this code will print the following:
    // न
    // म
    // स
    // ्
    // त
    // े
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // this code will print the following:
    // 224
    // 164
    // 168
    // 224
    // 164
    // 174
    // 224
    // 164
    // 184
    // 224
    // 165
    // 141
    // 224
    // 164
    // 164
    // 224
    // 165
    // 135
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}