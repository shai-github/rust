use std::fmt;
use std::io;


mod back_of_house1 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // can't create struct directly if this is not public
    }

    impl Breakfast {
        // public constructor
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        } 
    }
}

pub fn eat_at_restaurant () {
    // order a breakfast in the summer with rye toast
    let mut meal = back_of_house1::Breakfast::summer("rye");
    // change our mind about what bread we want
    // need to lable toast field as public to do this
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
}   

// another example using enums
// by default all variants of enums are public if you name an enum public
mod back_of_house2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let order1 = back_of_house2::Appetizer::Soup;
    let order2 = back_of_house2::Appetizer::Salad;
}

// example of use keyword
// use keyword brings path into scope
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// bring hosting module into scope
// idiomatic way is to bring parent module into scope
// this is to be explicit about where the function is defined
use crate::front_of_house::hosting;

pub fn eat_at_restaurant3() {
    // can use hosting without specifying full path
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// example of bring in functions with the same name from different modules
// specify result type using the parent modules first
// could also rename when bringing module into scope (e.g., use std::fmt::Result as FmtResult)
fn function1() -> fmt::Result {}
fn function2() -> io::Result<()> {}
