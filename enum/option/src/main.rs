/*
The option enum

Many languages have a null value, but Rust does not. Instead, Rust has an option enum.
The option enum forces you to handle the None case and guarantee in the Some case that
a value is present. This is inferred by default for type.

enum Option<T> {
    Some(T), // stores some generic value
    None, // stories no value
}
*/

fn main() {
    // what happens when you try to add an opton enum?
    let x: i8  = 5;
    let y: Option<i8> = Some(5);

    // this will not work because you cannot add an i8 and an Option<i8>
    // let sum = x + y;

    // to make code work, extract integer out of the sum variant
    // we can use unwrapping to handle the case when y is None, and also
    // define some sort of default value to use with unwrap_or
    let sum = x + y.unwrap_or(0);

    println!("The value of sum is: {sum}");

    let y: Option<i8> = None;
    let sum = x + y.unwrap_or(0);

    println!("The value of sum is: {sum}");
}
