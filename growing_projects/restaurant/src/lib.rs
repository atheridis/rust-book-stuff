#![allow(dead_code)]

// This is the name of the .rs file.
mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // impl doesn't need to be public
    impl Breakfast {
        // We need a public function to construct Breakfast
        // since seasonal_fruit is private
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// Before being moved
/*
pub mod front_of_house {
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

// Absolute path
use crate::front_of_house::hosting;

// Relative path with as keyword
use self::front_of_house::hosting as host;
*/

// You can still use Absolute path
use crate::front_of_house::hosting;

use front_of_house::hosting as host;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // with use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
    // using the as keyword
    host::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next lines won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal

    // meal.seasonal_fruit = String::from("blueberries");

    // let meal2 = back_of_house::Breakfast {
    //     toast: String::from("test"),
    //     seasonal_fruit: String::from("hi"),
    // };
}
