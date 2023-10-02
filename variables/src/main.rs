fn main() {
    // variables are immutable by default
    let mut x = 5;
    println!("The value of x is: {x}");

    // if mut was not used this would be an error
    x = 6;
    println!("The value of x is: {x}");

    // use const keyword for constant variables
    // you cannot mutate a constant variable
    // constant variables must be type annotated
    // constant variables can only be set to constant expressions
    // so, cannot assign to anything computed at runtime
    const MY_COUNT: u32 = 100_000;
    println!("The value of MY_COUNT is: {MY_COUNT}");

    // shadowing allows you to create a new variable using an existing name
    // you do this by using let again
    // benefits are that both declarations of the variable are immutable and
    // we can change the type of the variable
    let y = 7;
    println!("The value of y is: {y}");

    let y = "seven";
    println!("The value of y is: {y}");
}
