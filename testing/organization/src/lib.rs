/*
Unit tests are small, focused, and test one module in isolation at a time, and can test private interfaces
Integration tests are entirely external to your library and use your code in the same way any other external 
code would, using only the public interface and potentially exercising multiple modules per test.
*/

// this is public
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// this is private
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// cfg stands for configuration and we say that cargo will only compile
// this code wnen we run test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
