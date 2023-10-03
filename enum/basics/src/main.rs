/*
Enums allow us to enumerate a list of variants
*/


// we can define what we want to store in a variant
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// this helps us avoid defining multiple structs fof each variant
enum Message {
    Quite, // stores no data
    Move { x: i32, y: i32 }, // stores anonymous struct
    Write(String), // stores single String
    ChangeColor(i32, i32, i32), // stores three i32 values
}

impl Message {
    fn call() {
        println!("Hello, world!");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // variants are namespaced under their identifier
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddr {
        kind: IpAddrKind::V4(String::from(127, 0, 0, 1)),
    }
}

// we could, in this case, define a function that takes in an IpAddrKind and
// capture the version of the IP address and the actual IP address using a struct
fn route(ip_kind: IpAddrKind, ip_address: String) { 

}