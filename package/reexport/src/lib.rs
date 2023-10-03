/*
use keyword allows us to bring into scope but also allows external modules to come into scope
*/

// can use nested paths to bring multiple modules into scope from same parent
use rnd::{Rng, CryptoRng, ErrorKind::Transient};

// use self to refer to io itself
// to bring all std::io into scope, use the glob operator
// use std::io::*;
use std::io::{self, Write};

// this tells rust to find front_of_house module but to get contents
// from file that has same name as module
mod front_of_house;

// to reexport, add pub keyword in front of use statement
// this brings hosting module into scope and marking as public
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant3() {
    // can use external module brought into scope without specifying full path
    let secret_number = rnd::thread_rng().gen_range(1, 101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}