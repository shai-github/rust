pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// test is already written in the library file
#[cfg(test)]
mod tests {
    use super::*;

    // in Rust, functions are tests if they have the #[test] attribute
    // can also have other helper functions that are not tests
    // can run this test with cargo test
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // add a test to see that this fails
    #[test]
    fn it_fails() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }
}
