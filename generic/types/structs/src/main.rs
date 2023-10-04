/*
We can use generics with structs
*/


// add generics to the point type
// we say x is going to be of a generic type T and y is going to be of a generic type U
// they could be the same generic type but we could expect them to be different types
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };

    // what if we want to create a point with a float rather than i32?
    // we can use generics to accomplish this
    let p2 = Point { x: 5.0, y: 10.0 };

    // what if x is an integer and y is a floating point number?
    // this would error for mismatched types
    // we can have multiple generic types defined in the struct
    let p3 = Point { x: 5, y: 10.0 };
}
