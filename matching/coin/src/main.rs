/*
Match expression allows you to compare a value against a set of patterns.
Match expression is exhaustive such that we have to match all possible cases.
This makes match useful for enums.
*/

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    //...
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

// return value for every type of coin defined in enum Coin
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1 
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,

        // match state on quarter coin variant
        // state variable will bind to US State on quarter
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}