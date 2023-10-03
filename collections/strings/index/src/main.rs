use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let hello: String = String::from("Hello");

    // we want to get the first character of the string
    // we cannot simply index by an integer because a string is a
    // collection of bytes, not characters
    // 5 characters may not necessarily be 5 bytes in length
    // let c: char = hello[0];

    // let's break it down using another language
    let hello: String = String::from("नमस्ते");

    // Bytes (18 bytes in this word)
    // [224 164 168 224 164 174 224 164 184 224 165 141 224 164 164 224 165 135]

    // Scalar Values (char type in Rust)
    // ['न', 'म', 'स', '्', 'त', 'े']

    // Grapheme Clusters (what we would think of as letters)
    // ["न", "म", "स्", "ते"]

    // Rust does not know what type we want to receive the string in
    // We need to use more specific methods
    // can use the bytes method to access bytes in the string
    for b in hello.bytes() {
        println!("{b}");
    }

    // use chars method to iterate over scalar values
    for c in hello.chars() {
        println!("{c}");
    }

    // cannot iterate over grapheme clusters by default in the standard library
    // need to import a crate in order to do this (unicode-segmentation in Cargo.toml)
    for g in hello.graphemes(true) {
        println!("{g}");
    }
}
