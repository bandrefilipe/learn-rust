#[allow(dead_code)]
pub fn execute() {
    defining_and_instantiating_structs();
}

fn defining_and_instantiating_structs() {
    instantiate_a_user();
    build_user("someone@example.com".to_string(), "someone".to_string());
    creating_instances_from_other_instances();
    tuple_structs();
}

#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn instantiate_a_user() {
    // note that the entire instance must be mutable so we can mutate some field
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // assigning a new value to email
    user.email = String::from("someone@new-email.com");
}

/// A `build_user` function that uses field init shorthand because the `email` and `username` parameters have the same name as struct fields
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn creating_instances_from_other_instances() {
    let user1 = User {
        username: String::from("username1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
        active: true,
    };
    let _user2 = User {
        username: String::from("username2"),
        email: String::from("user2@example.com"),
        ..user1 // the remaining fields not explicitly set should have the same value as the fields in the given instance
    };
}

fn tuple_structs() {
    struct Color(u8, u8, u8);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}
