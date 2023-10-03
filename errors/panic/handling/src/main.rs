/*
To get backtrace: RUST_BACKTRACE=1 cargo run
*/

fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(42);
}

fn c(num: i32) {
    if num == 42 {
        panic!("42 is not allowed!");
    }
}