/*
Subdirectors in tests folder will not get compiled as separate crates. You will not
see tests for this module in the output of cargo test.

Using mods.rs allows us to use module in our integration tests.
*/

pub fn setup() {
    // setup code for example
}