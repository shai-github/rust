/*
In Rust, strings are stored as a collection of UTF-8 encoded bytes. In memory, a string
is a collection of binary data and UTF-8 encodes this binary in memory.

With UTF-8, character bytes can range from 1 to 4. 
*/

fn main() {
    // create an empty string
    let s1 = String::new();

    // create a string slice
    let s2 = "initial contents";

    // turn a string slice into a string
    let s3 = s2.to_string();

    // create own string into a string slice
    let s4 = String::from("initial contents");

    // how about appending to a string? 
    // a vector can grow or shrink in size
    let mut s = String::from("foo");

    // method takes a string slice rather than ownership
    s.push_str("bar");

    // can also use push method
    s.push('!');

    // append strings using the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // s1 has been moved here and can no longer be used
    // saves memory compared to moving both strings
    // we can no longer borrow a value after it has been moved
    let s3 = s1 + &s2;

    // can concatenate using format macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");

    // this combines s1 and s2 and stores in s3
    // format macro does not take ownership, so they can be used after the call
    let s3 = format!("{}-{}", s1, s2);

    println!("{s3}",);
}
