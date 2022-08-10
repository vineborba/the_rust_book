// common path -> rand::...
use rand::{CryptoRng, ErrorKind::Transient, Rng};

// self -> io, Write -> io::Write
// use std::io::{self, Write};

// all of io
// use std::io::*;
mod front_of_house;


fn serve_order() {
    println!("Serving order");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()
    }

    fn cook_order() {
        println!("Cooking order")
    }

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

// abosulute path
// use crate::front_of_house::hosting;

// relative path
use self::front_of_house::hosting;

// re-exporting hosting module
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // abosulute path
    front_of_house::hosting::add_to_waitinglist();

    // relative path
    front_of_house::hosting::add_to_waitinglist();

    // short relative path (because of use)
    hosting::add_to_waitinglist();

    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheat");

    // Error -> seasonal_fruit is private
    // let meal2 = back_of_house::Breakfast {
    //     toast: String::from("wheat"),
    //     seasonal_fruit: String::from("apples")
    // };

    let _order = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    let secret_number = rand::thread_rng().gen_range(1, 101);
}
