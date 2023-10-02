/* 
The rules of references
1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
*/

fn main() {
    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function and so is no longer valid here
    // can clone to avoid this
    takes_ownership(s.clone());

    // this will throw an error if not cloned
    println!("{s}");

    // can still use x after the function call
    // this is because i32 is a Copy type rather than being moved
    let x = 5;
    makes_copy(x);
    println!("{x}");

    // returning will move ownership to s1
    let s1 = gives_ownership();
    println!("{s1}");

    // we can take and give back ownership
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{s1}, {s3}");

    // try references
    // use & to pass a reference to the value of s1
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");
    
    // mutable references
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{s1}");

    // mutable references have a significant restriction, which is that you can only 
    // have one mutable reference to a particular piece of data in a particular scope
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s // this would throw an error
    // this prevents a data race at compile time
    // fix error by using immutable references
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    // however, what happens when you mix mutable and immutable references?
    // let r3 = &mut s; // this would throw an error
    // immutable references do not expect underlying values to change
    // you can, however, have multiple immutable references in the same scope
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1}, {r2}"); // the scope ends here

    // end of scope means that we can add a third mutable reference after scope
    let r3 = &mut s;
    println!("{r3}");

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

// this function will return ownership of the string to the calling function
// returning the string moves ownership to s1 variable in main
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

// will take ownership and give it back by returning input value
// this moves value out of the function into assigned variable s3 in main
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// moving ownership and back out is tedious, so we can use references
// references do not take ownership of the underlying value
// this can be achieved by using tuples but we don't want to do that
// can instead use a reference with & symbol
// the input s is the reference that points to s1 which points to string in heap
// this concept is called borrowing, because we are borrowing rather than actually
// take ownership
// note that references are immutable by default
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

// this function will help us modify the value of a reference
// without changing ownership
// we can do this by using the mut keyword in main
fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

// what happens when there is a reference that points to invalid data
// returns a reference to a string
// when function finishes executing, rust will deallocate the string from the heap
// our reference will then be pointing to invalid data, so this would raise an error
// fn dangle() -> &String { 
//     let s = String::from("hello");

//     &s
// }
