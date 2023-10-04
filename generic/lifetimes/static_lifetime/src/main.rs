/*
Static lifetime means that the reference could live as long as the duration of the program.
All string literals have a static lifetime because they are stored in the binary of the program.
*/

fn main() {
    let s: &'static str = "I have a static lifetime.";
}