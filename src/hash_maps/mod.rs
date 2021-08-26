#[allow(dead_code)]
pub fn execute() {
    creating_a_new_hashmap();
    hash_maps_and_ownership();
    accessing_values_in_a_hash_map();
    updating_a_hash_map();
}

fn creating_a_new_hashmap() {
    // hash maps are not present in the prelude, so we need to bring it into scope:
    use std::collections::HashMap;

    // we can create hash maps this way:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // or by merging (zipping) two vectors into tuples and then collecting it to a hash map:
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<String, i32> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    assert_eq!(scores.get("Blue"), Some(&10));
    assert_eq!(scores.get("Red"), Some(&50));
}

/// Showing that keys and values are owned by the hash map once they're inserted.
fn hash_maps_and_ownership() {
    use std::collections::HashMap;

    let key = String::from("Favorite color");
    let value = String::from("Green");
    let mut map = HashMap::new();
    map.insert(key, value);
    // the following line won't compile, as both values have been moved into the map:
    // println!("{}={}", key, value);
}

fn accessing_values_in_a_hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 50);

    let team_name = "Blue".to_string();
    let score = scores.get(&team_name);
    assert_eq!(score, Some(&10));

    // iterating over each key-value pair in a hash map:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn updating_a_hash_map() {
    overwriting_a_value();
    only_inserting_a_value_if_key_has_no_value();
    updating_a_value_based_on_the_old_value();
}

fn overwriting_a_value() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);
    scores.insert("Blue".to_string(), 25);

    assert_eq!(scores.get("Blue"), Some(&25));
}

/// Using the `entry` method to only insert if the key does not already have a value
fn only_inserting_a_value_if_key_has_no_value() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue".to_string(), 10);

    scores.entry("Yellow".to_string()).or_insert(50);
    scores.entry("Blue".to_string()).or_insert(50);

    assert_eq!(scores.get("Yellow"), Some(&50));
    assert_eq!(scores.get("Blue"), Some(&10));
}

/// Counting occurrences of words using a hash map that stores words and counts.
fn updating_a_value_based_on_the_old_value() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    assert_eq!(map.get("world"), Some(&2));
    assert_eq!(map.get("hello"), Some(&1));
    assert_eq!(map.get("pretty"), None);
}
