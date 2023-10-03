fn main() {
    let a = [1, 2, 3, 4, 5];

    // make empty vector
    // cannot infer type of empty vector, so must annotate
    // make mutable to add elements
    let mut v: Vec<i32> = Vec::new();

    // use push to add to vector
    v.push(1);
    v.push(2);
    v.push(3);

    // create vector and initialize with values
    // type is inferred from type passed into macro
    { // place in its own scope
        let v2 = vec![1, 2, 3];
    } // v2 and elements inside it are dropped here

    // let's see how we can access vector elements
    let v3 = vec![1, 2, 3, 4, 5];

    let third = &v3[2];
    println!("The third element is {third}");

    // to avoid crashes when accessing elements out of bounds use get method
    // use match to handle Some and None case
    match v3.get(2) {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // there is no element at index 20, so this will return None
    match v3.get(20) {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // recall that you cannot have a mutable and immutable reference to a 
    // single element at the same time - how do you handle this with vectors?
    // first, push another value to v4
    let mut v4 = vec![1, 2, 3, 4, 5];
    let third2 = &v4[2];

    // cannot borrow v as mutable
    v.push(6);

    // this would return an error because of the immutable borrow above
    // println!("The third element is {third2}");
}
