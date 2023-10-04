/*
The lifetime of a return value always needs to be tied to the lifetime of one of the parameters.
This is because if we pass back a reference from a function, there must be a reference to 
something that is passed in. 
*/

fn main() {
    let string1 = String::from("abcd");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is {}", result);
}

// in this case we don't care about the lifetime of y
// we know that longest will always return lifetime of x
// string1 lives until the end of the main function, so this is valid
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// this would not work because there is no lifetime passed in for
// the return value with a lifetime to be valid
// we cannot return a reference to something created within the function
// because once the function is over, the local variables are destroyed
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// we can instead return an own type
// here we return a String rather than a reference to a string slice
// returning an own type transfers ownership, so this is valid
fn longest<'a>(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result.as_str()
}