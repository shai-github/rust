/*
Tuple structs do not have named fields. They are useful when you want to give 
the whole tuple a name and be of different types than other tuples.
*/

fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}
