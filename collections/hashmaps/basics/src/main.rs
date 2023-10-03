/*
Hash maps allow you to store key value pairs (like a dictionary in python)
*/

use std::collections::HashMap;

fn main() {
    // to create a new hash map, first need to bring type into scope
    // we want keys to be team name and values to be the team score
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // moves ownership of strings into the hash map
    // will no longer be able to borrow the moved value
    // could pass a reference using lifetimes
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // use get method to access values
    let team_name = String::from("Blue");

    // this returns an optional value because we cannot guarantee that a
    // value will be returned
    let score = scores.get(&team_name);

    // iterate over hash map using for in loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // how do you update the hash map?
    let mut scores2 = HashMap::new();

    // insert values to key by overwriting
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 20);

    // to avoid overwriting values, use entry method
    // call or_insert method to insert entry if a value does not exist
    // if there is an entry for the key, do nothing (30 will work, 40 won't)
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);

    // how do you update a value in a hash map based on an old value?
    // we want to populate the hash map with the word count of a string
    // for example, key = hello, value = 1 and key = world, value = 2
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // entry returns an enum representing the value of that key, which could exist or not
        // or_insert returns a mutable reference to the value for this key
        // we can dereference the mutable reference to change the value
        // on the second instance of "world", or_insert will not do anything but we will still
        // get back a mutable reference to the value 1, which we can increment by 1 to get 2
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
