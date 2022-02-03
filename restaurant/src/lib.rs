mod front_house {
    pub mod hosting {
        pub fn add_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_house::hosting::add_waitlist();

    // Relative path
    front_house::hosting::add_waitlist();
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

use crate::front_house::hosting;

// Idiomatic use path
//use crate::front_of_house::hosting::add_to_waitlist;

use std::collections::HashMap;

//Providing new names with the 'as' keyword
use std::io::Result as IoResult;

// Nester paths to avoid large use lists
use std::io::{self, Write};
use std::{cmp::Ordering, io};

// use the Glob operator;
use std::collections::*;

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_waitlist();
    // Idiomatic use path
    //add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);

    //IoResult::from_iter(iter: I)
}
