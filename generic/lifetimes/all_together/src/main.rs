use std:fmt::Display;

// function takes two string references and generic call ann
// it will then print the announcement and return the longer of the two strings
// ann is of type T, so it is generic
fn longest_with_an_announcement<'a, T>(
    // because we have two references, the compiler cannot complete automatic
    // lifetime elision, so we have to specify the lifetimes on the parameters
    // and return value
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
// use a trait bound to limit ann to types that implement Display
where 
    T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}