/*
A benefit of generics is that they allow us to reduce duplication.
Rather than define to versions of an Option enum, we can use generics to define one enum.
This does not incur a performance hit, because at compile time, Rust will turn the Option
enum into two option enums with the specific types we use (e.g., integer or float)
*/

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let integer = Option::Some(5);
    let float = Option::Some(5.0);
}