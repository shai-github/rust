/*
Vectors can only store one type of data but we could use enums to store 
different types of data
*/

fn main() {
    // each row can store some type of data
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // create a vector of type SpreadsheetCell with different row types
    // this allows for storage of different types of data in a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    // need to use a match expression to get the value out of the enum reference
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("Int: {}", i),
        _ => println!("Not an integer"),
    }
}
