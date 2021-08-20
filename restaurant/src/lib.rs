mod front_of_house {

    pub mod hosting {

        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {

        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// the use of `pub` keyword here denotes it is being re-exported.
// meaning the caller module can reference simply `restaurant::hosting::add_to_waitlist()`
// instead of knowing anything about front our back of house concepts.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path:
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path:
    front_of_house::hosting::add_to_waitlist();

    // after bringing hosting into scope with `use`:
    hosting::add_to_waitlist();

    // order a breakfast in the summer with rye toast:
    let mut meal = back_of_house::Breakfast::summer("rye");
    // change our mind about what bread we'd like:
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // the next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


fn serve_order() {}

mod back_of_house {
    
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();        
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
