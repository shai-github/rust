/*
Generic lifetime annotations describe the relationship between the lifetimes of multiple references
and how they relate to each other - they explain the relationship rather than change lifetimes.
People generally refer to generic lifetime annotations simply as lifetimes. 

Convention is to name 'a, 'b, 'c, etc.

Different types of references relative to lifetimes:
1. &i32         // a reference
2. &'a i32      // a reference with an explicit lifetime
3. &'a mut i32  // a mutable reference with an explicit lifetime
*/

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    // result variable is set equal to the longest function
    let result = longest(string1.as_str(), string2.as_str());

    // how would the borrow checker know whether the result is not a dangling reference?
    // the borrow checker checks lifetimes of references but what is the lifetime of result?
    // how does the borrow checker know that result is not a dangling reference?
    // we told the borrow checker that whatever is returned from the longest function
    // will live at least as long as the smaller of the lifetimes of string1 and string2
    println!("The longest string is: {}", result);

    // but what is strings have different lifetimes?
    let string3 = String::from("abcd");

    // everything else defined inside the inner scope
    // string4 lifetime only lasts as long as the inner scope
    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is: {}", result2);
    }

    // a final example
    let string5 = String::from("abcd");
    let result3;

    {
        let string6 = String::from("xyz");
        
        // string 6 in this case does not live long enough
        // result3 lifetime only lasts until end of inner scope
        result3 = longest(string5.as_str(), string6.as_str());
    }

    // printing after inner scope is problematic because string6 does not live long enough
    // for this statement to be valid - a case of a dangling reference
    println!("The longest string is: {}", result3);
}

// return type here is a reference to a string slice
// borrow checker needs to know where lifetime is borrowed from, so we use generic lifetime annotations
// we specify a lifetime after the function name 
// we then specify that x and y have the same lifetime 'a
// return type will use the same lifetime 'a
// generic lifetime annotations don't change the lifetimes, but they do create relationships between
// the lifetimes of multiple references
// here, we are essentially saying that the lifetime of the return reference is the same as the
// lifetime of the smaller of the lifetimes of the references passed in
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // x and y could have a different lifetime and the borrow checker would not know
    // which lifetime to use for the return value given that neither x nor y return is guaranteed
    // we also do not know the exact lifetimes and these are just placeholders
    // borrow checker cannot handle this ambiguity
    if x.len() > y.len() {
        x
    } else {
        y
    }
}