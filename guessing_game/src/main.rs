use std::io;


fn main() {
    // prompt user with message
    println!("Guess the number!");
    println!("Please input your guess.");

    // create string variable to store user input
    // new is an associative function (like a static method)
    // new returns an empty string we can use
    // use mut to make the variable mutable
    let mut guess: String = String::new();

    // take a reference to string and modify without taking ownership
    // readline returns a result type which is an enumeration of OK
    // or Err to handle the error case
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
