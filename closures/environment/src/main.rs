/*
Unlike functions, closures have access to variables that are defined within the scope
in which the closure is defined

FnOnce, FnMut, Fn
FnOnce: consumes the variables it captures from its enclosing scope, known as the closure’s environment.
FnMut: can change the environment because it mutably borrows values.
Fn: borrows values from the environment immutably.
*/

fn main() {
    let x = 4;

    // even though x defined outside of closure, it is available inside closure
    // because defined in the same scope
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
