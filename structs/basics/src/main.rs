/*
Structs allow you to group related attributes together.
Can think of structs as object attributes in OOP
*/

// Define a struct for a user
// assign attributes to the struct
// like a tuple, this allows us to group together related data of different types
// allows us to reference data by name rather than by index
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // an instance of the User struct
    let mut user1 = User {
        email: String::from("newuser@mail.com"),
        username: String::from("user1")
        active: true,
        sign_in_count: 1
    };

    // get specific attributes using dot notation
    let name = user1.username;

    // modify specific attributes using dot notation
    // make sure that the user1 variable is mutable
    user1.username = String::from("newuser");

    // use function to create another user
    let user2 = build_user(
        String::from("anotheruser@mail.com"),
        String::from("user2")
    );

    // use an existing instance to create a new instance
    // this is accomplished using ..user2 after other inputs
    let user3 = User {
        email: String::from("yetanotheruser@mail.com")
        username: String::from("user3")
        ..user2
    }
}

// this functions uses the field init syntax to create a new user
// note that email and username do not need to be explicitly specified
// this is because the inputs are the same names as the attributes
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
