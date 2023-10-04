/*
Can implement a trait on a type taht implements another trait
*/

// Implement ToString trait on any type that implements Display
// used widely in std library
impl<T: Display> ToString for T {
    // --snip--
}
