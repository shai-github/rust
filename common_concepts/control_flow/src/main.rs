fn main() {
    let number = 3;

    // example of conditional flow
    if number < 5 {
        println!("First condition was true");
    } else if number < 7 {
        println!("Second condition was True");
    } else {
        println!("Condition was false");
    }

    // example of conditional flow using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number} because condition is {condition}");
}