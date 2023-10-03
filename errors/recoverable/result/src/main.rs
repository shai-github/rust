/*
Result enum is similar to the Option enum, representing success or failure.

enum Result<T, E> {
    Ok(T),
    Err(E),
}

Enum and variants are brought into scope because of how common they are.
*/

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // this is of form Result<File, Error>
    let f = File::open("hello.txt");  

    // use match expression to handle success or failure cases
    // rather than return an error in the case that the file does not exist,
    // we want to create the file
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() { // want to handle not found error case
            ErrorKind::NotFound => match File::create("hello.txt") { 
                Ok(fc) => fc, // if file not found, attempt to create file
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => { // handle other errors
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    // however, using nested match expression can be difficult to read
    // instead, use closures
    // unwrap_or_else will give us back the file or call the anonymous function (closure)
    // if error not found, we attempt to create the file calling unwrap_or_else again
    // in failure case, another closure will panic and will also panic in the else case
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound { // if file not found, attempt to create file
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else { // handle other errors
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
