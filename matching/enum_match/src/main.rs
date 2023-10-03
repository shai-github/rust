fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// take in optional integer and return optional integer
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        Some(i) => Some(i + 1), // must wrap in Some because return value is optional
        _ => None, // any other pattern, execute this
    }
}