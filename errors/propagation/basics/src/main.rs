/*
When you have a function whose implementation calls something that returns something that can fail,
you want to give more control to the caller in order to decide what to do
*/

use std::fs::{self, File};
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    // attempt to open file
    // ? is the same as handling with a match expression
    // let met f = match f {
    //     Ok(file) => file,
    //     Err => return Err(e),
    // };
    let mut f = File::open("hello.txt")?;

    // create new mutable string
    let mut s = String::new();

    // read content of file and store in string
    // allows code calling functions how to best deal with errors
    // this can be simplified as show below with ?
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s), // return Ok variant of Result enum
    //     Err(e) => Err(e), // return Err variant of Result enum
    // }
    f.read_to_String(&mut s)?;
    Ok(s)
}

fn read_username_from_file_chain() -> Result<String, io::Error> {
    // ? can be used in a chain to simplify code
    // this is the same as the code above
    let mut s = String::new();

    // here, we open our file and specify ? to return error to caller if Err
    // specify ? again to return error to caller if Err
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// can be simplified even further
fn read_username_from_file_short() -> Result<String, io::Error> {
    // simplify call to read_to_string from fs module
    fs::read_to_string("hello.txt")
}

fn main() {}
