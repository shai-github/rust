/*
This can be made by default using cargo new --lib restaurant
Creates a test module by default but not needed for this example
*/


// specify module using mod keyword
// modules can contain other modules inside of them 
// can also contain structs, enums, constants, traits, etc.
// structuring code this way keeps it organized
mod front_of_house_example {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// we can simplify the module
mod front_of_house {
    // expose hosting module using pub keyword
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// by default, child module is private from perspective of parent module
// child modules are able to see anything defined in parent modules
// we can hide implementation details by default
pub fn eat_at_restaurant() {
    // absolute path
    // starts at root of module tree crate
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // starts from current module 
    front_of_house::hosting::add_to_waitlist();
}

// example of relative paths using super keyword
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super keyword allows us to go up one module (crate)
        // so, we can call serve_order() from parent module
        super::serve_order();
    }

    fn cook_order() {}
}
