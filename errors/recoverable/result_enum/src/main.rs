/*
Rather than call a match expression, we can simplify code by calling unwrap
*/

use std::fs::File;

fn main() {
    // unwrap does the same thing as the match expression
    // let f = match f {
    //     Ok(file) => file, // if file exists, return file
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };  
    let f = File::open("hello.txt").unwrap();

    // can also use the expect method
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}