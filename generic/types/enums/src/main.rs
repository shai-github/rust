/*
The two most popular enums, Option and Result, are implemented using generics
*/

fn main() {
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
