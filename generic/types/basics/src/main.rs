/*
Generics, traits and lifetimes are all used to reduce code duplication
*/

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // we set the first number in vector as reference
    let mut largest = number_list[0];

    // check each number in vector against the reference
    // if a numbe is greater than the reference, it becomes the reference
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is: {largest}");

    // what if we want to find the largest number in a different vector?
    // the logic would be the same but we do not want to duplicate the code
    // turn the code into a function (get_largest)
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = get_largest(number_list);

    println!("The largest number is: {largest}");

    // let's pass in a char list
    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest2(char_list);

    println!("The largest char is: {largest}");

}

// this is a good first step, but this function is tied to a concrete set of arguments
fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// what if we want to use the same logic over a slightly different set of arguments?
// say we want to find the largest character in a vector
// we can use generics to accomplish this using <T> or <Type>
// use PartialOrd and Copy traits to allow for comparison
fn get_largest2<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        // can't use greater operator on type <T>
        // we need to restrict generic type to fix this
        // generic must be of any type that can be compared using traits
        if number > largest {
            largest = number;
        }
    }

    largest
}
