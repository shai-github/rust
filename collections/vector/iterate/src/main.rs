fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // to iterate over all elements and print, use for in loop and what
    // happens here is that we loop through and take an immutable reference
    for i in &v {
        println!("{i}");
    }

    // we could also take a mutable reference
    // use dereference operator * to get value in i
    for i in &mut v {
        *i += 50;
    }

    // print again to see that values have changed
    for i in &v {
        println!("{i}");
    }
}
