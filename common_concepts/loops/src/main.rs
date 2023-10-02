fn main() {
    // will execute code until break is called
    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is: {result}");

    // the while loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // the for loop
    let a = [10, 20, 30, 40, 50];

    // iter will give an iterator for the array
    for element in a.iter() {
        println!("The value is: {element}");
    }

    // can also use to iterate over a range
    // will print in reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!! Again!");
}
