fn main() {
    // Compound types
    let tup = ("Let's start", 100_000);

    // destructuring
    let (message, count) = tup;
    println!("{}: {}", message, count);

    // use dot notation
    let dot_message = tup.0;
    let dot_count = tup.1;
    println!("{}: {}", dot_message, dot_count);

    // arrays
    // arrays are fixed length
    // dynamic length requires vectors
    let error_codes = [200, 404, 500, 501, 502, 503, 504, 505];

    // access element by position using standard array notation
    let not_found = error_codes[1];
    println!("Not found: {}", not_found);

    // create array of 8 elements with each element initialized to 0
    let byte = [0; 8];
    println!("Byte: {:?}", byte);
}
