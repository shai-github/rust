fn main() {
    // a is executed first so we push a onto the stack
    // then b is executed by a, so we push stack frame for b onto the stack
    // when b is done executing, its local variables are dropped, then same for a
    // the heap is less organized than the stack
    fn a() {
        let x = "hello";
        let y = 22;
        b();
    }

    // we ask heap to allocate memory for the string and heap passes back pointer
    // the pointer is actually what is stored on the stack
    fn b() {
        let x = String::from("world");
    }
}