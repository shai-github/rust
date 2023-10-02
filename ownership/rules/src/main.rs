fn main() {
    let x = 5;
    let y = x; // x is copied to y

    println!("x = {x}, y = {y}");

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, not a shallow copy   

    println!("{s2}, world!")
}

fn ownership_rules() {
    /*
    Ownership Rules
    1. Each value in Rust has a variable that’s called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */ 

    // Take the follow example of the variable s
    { // s is not valid here, it’s not yet declared
        let _s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid
}
