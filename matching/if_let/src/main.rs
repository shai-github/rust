fn main() {
    let some_value = Some(3);

    match some_value {
        Some(3) => println!("three"), // if exactly 3, print three
        _ => (), // otherwise do nothing
    }

    // let's rewrite using if let syntax
    // read this as: if some_value matches Some(3), execute the code block
    // this lets us write less code rather than be exhaustive for option value
    if let Some(3) = some_value {
        println!("three");
    }
}
