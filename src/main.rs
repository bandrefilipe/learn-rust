mod branches;
mod collections;
mod data_types;
mod enumerations;
mod error_handling;
mod functions;
mod generics;
mod guessing_game;
mod hash_maps;
mod hello_world;
mod lifetimes;
mod loops;
mod ownership;
mod pattern_matching;
mod rectangles;
mod strings;
mod structs;
mod traits;
mod variables;

fn main() {
    // hello_world::execute();
    // guessing_game::execute();
    // variables::execute();
    // data_types::execute();
    // functions::execute();
    // branches::execute();
    // loops::execute();
    // ownership::execute();
    // structs::execute();
    // rectangles::execute();
    // enumerations::execute();
    // pattern_matching::execute();
    // collections::execute();
    // strings::execute();
    // hash_maps::execute();
    // error_handling::execute();
    // generics::execute();
    // traits::execute();
    lifetimes::execute();
}

#[cfg(test)]
mod tests {

    /// Tests that return a [Result] type allow us to use the `?` operator
    /// which can be convenient if we have multiple operations within the test that
    /// could return an [Err] type, and we want our test to fail if any Err type is returned.
    #[test]
    fn test_returning_result_type() -> Result<(), String> {
        if two_plus_three() == 4 {
            Ok(())
        } else {
            Err(String::from("two plus three does not equal four"))
        }
    }

    fn two_plus_three() -> i32 {
        // 2 + 3 // commented out so the test above doesn't fail
        2 + 2
    }
}
