#[allow(dead_code)]
pub fn execute() {
    generic_lifetimes_in_functions();
    lifetime_annotations_in_struct_definitions();
}

fn generic_lifetimes_in_functions() {
    let result = largest("abcd", "xyz");
    assert_eq!(result, "abcd");

    let first = returns_first("first", "second");
    assert_eq!(first, "first");
}

fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

fn returns_first<'a>(first: &'a str, second: &str) -> &'a str {
    println!("second: {}", second);
    first
}

fn lifetime_annotations_in_struct_definitions() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find any '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    assert_eq!(excerpt.part, "Call me Ishmael");
}
