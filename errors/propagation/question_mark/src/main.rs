/*
If we try to use a question mark operator in the main function, we have to remember that
the main function does not return anything - there are restrictions on what type it can return.
The main function can return a none type or a result type.
*/

user std::error::Error;
use std::fs::File;

// change main to return a result type
// if success, return a unit (essentially nothing)
// if error, return a trait object that implements the Error trait
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
