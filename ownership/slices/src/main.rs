/*
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
Slices do not take ownership of the underlying data
*/

fn main() {
    let mut s = String::from("hello world");

    // use string slices to avoid errors
    // gives reference to part of string by index
    // s has a pointer to the string on the heap and
    // hello and world point to same string on heap but start at different index
    let _hello = &s[..5];
    let _world = &s[6..];
    let _both = &s[..];

    // cannot mix mutable and immutable string so let's set a new variable
    // we can use a string slice that is a reference to the string literal in the binary
    // using this means that the function needs to take in a string slice with &str rather than &String
    let s2 = "hello world";

    let word = first_word(s2);
    println!("The first word of '{s2}' is: {word}");

    // slices can also exist on different types of collections
    // let's create a new variable called slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];

    println!("The first two elements of {:?} are: {:?}", a, slice);
}

// we turn an index to the end of a word
// we convert string to an array of bytes
// call enumerate to get index and value of each byte
// we look for the first space byte to indicate end of a word
// not finding a string means that the string is one word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // return string slice that references the string itself    
    &s[..]
}
