use std::io;
use std::cmp::Ordering;
use rand::Rng;  
use colored::*;


fn main() {
    // prompt user with message
    println!("Guess the number!");

    // generate random number between 1 and 100
    // thread_rng returns a random number generator
    // gen_range is a method that will produce a number
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    // use loop so that user can keep guessing the number
    loop {
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

        // trim will parse white space from the string
        // parse will turn string into an integer using u32
        // use match to handle possible error variants
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a catchall value
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compare guess to secret number
        // cmp takes a reference to the varible we want to compare
        // cmp returns an Ordering enum which has variants Less, Greater, Equal
        // use match expression to decide what to do next based on cmp result
        match guess.cmp(&_secret_number) {
            // use color package to make output more readable
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            // we want the game to end if the user guesses correctly
            // we do this by using break to exit the loop
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}
