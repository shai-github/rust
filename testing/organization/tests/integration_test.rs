/*
Cargo knows to look for integration tests in the tests directory.

You can run integration tests with the command cargo test --test integration_test.
After running cargo test, you’ll see that the integration test is run in addition to the unit tests.

running 1 test
test tests::internal ... ok

running 1 test
test it_adds_two ... ok
*/

use organization; // need to bring outer library into scope

// use mod from common/mod.rs
mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, organization::add_two(2));
}