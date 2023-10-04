/*
If we want to use a reference with structs, we need to specify a lifetime annotation
*/

// lifetime annotation in struct definition says that our struct cannot
// outlive past the reference passed into part attribute
struct ImportantExcept<'a> {
    // specify a lifetime annotation here
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcept { 
        part: first_sentence 
    };
}