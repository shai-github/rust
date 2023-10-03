/*
You can use expect or unwrap when you know a call to a function will succeed 
*/
use_std::net::IpAddr;

fn main() {
    // string is hardcoded so we know parse will succeed, so we can use unwrap
    // otherwise, we would want to handle the error case (e.g., user input)
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}
