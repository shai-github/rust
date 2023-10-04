/*

*/

use std::fmt::Display;

// struct has two fields that are both generic type T
struct Pair<T> {
    x: T,
    y: T,
}

// impl of Pair that only applies to Pair<T> 
impl<T> Pair<T> {
    // creates a new pair
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// similar to above but we use trait bounds that specify that
// the generic type T must implement Display + PartialOrd traits
impll<T: Display + PartialOrd> Pair<T> {
    // method will only be available to Pair structs where the type
    // of x and y implement Display and PartialOrd traits
    fn cmp_display(&self) {
        // we can compare the two members of the struct because
        // we know that they implement the Display and PartialOrd trait
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}