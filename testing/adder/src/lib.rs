/*
You can run tests with cargo test command.
You can also use cargo test -- --test-threads=1 but test will run slower. May be useful in cases where
you want to run tests in serial fashion. 

Can use command cargo test -- --show-output
This will show print statement for both passing and failing cases

To run only one test, you can use the command cargo test <test_name>.
To run only tests that match a particular name, you can use the command cargo test <part_of_test_name>.
 */

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
