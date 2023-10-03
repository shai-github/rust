/*
We can create a custom error type to use in validation, for example
*/


// Guess struct stores integer
pub struct Guess {
    value: i32,
}

impl Guess {
    // takes in value, validates, and returns Guess struct
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // returns value stored in Guess struct
    pub fn value(&self) -> i32 {
        self.value
    }
}