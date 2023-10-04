pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Riley");
        assert!(
            result.contains("Sam"),
            // custom error message
            "Greeting did not contain name Sam, value was `{}`",
            result
        );
    }
}
