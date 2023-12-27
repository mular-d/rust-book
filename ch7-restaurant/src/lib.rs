mod front_of_house;

use crate::front_of_house as front;
pub use crate::front_of_house::hosting;

fn deliver_order() {}

mod back_of_house {
    // making structs and enums public
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

    // starting relative paths with super
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let _order1 = back_of_house::Appetizer::Soup; // fields of enum are public by default
        let _order2 = back_of_house::Appetizer::Salad;
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();

    // local name alias
    front::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // doesn't work cause its a private field
}
