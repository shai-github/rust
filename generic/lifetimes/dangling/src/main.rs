/*
Dangling references point to invalid data, and Rust does not like references
*/

fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    // r is a dangling reference because r is a reference to x declared
    // inside the inner scope and x is dropped when the inner scope ends
    // rust will not allow this code to compile because x "does not live long enough"
    // this is known because of the borrow checker, which runs at compile time
    // and checks that all borrowed values and references are valid
    println!("r: {}", r);

    // let's look at another example
    let x = 5

    // r is a reference to x and we will not get a compile time error
    // this is because the lifetime of the variables goes to the end of the function
    // the lifetime of x, essentially, remains valid for the reference to work
    // the borrow checker is smart enough to know this, but there are situations
    // where we need to help the borrow checker
    let r = &x
    println!("r: {}", r);
}
