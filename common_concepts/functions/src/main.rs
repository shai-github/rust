fn main() {
    let sum = my_function(11, 22);
    println!("The value of sum is: {sum}");
}

// this of function as an expression that returns a value
fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // can return a value from a function using the return keyword
    // return type must be assigned in order to use keyword
    x + y
}