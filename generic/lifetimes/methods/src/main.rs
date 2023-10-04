struct ImportantExcept<'a> {
    part: &'a str,
}

// just like with generics, we need to include lifetime annotation
// after impl and after name of the struct
impl<'a> ImportantExcept<'a> {
    // we don't need to specify lifetime annotations here because of elision rules
    // reference to self gets a lifetime and announcement gets a lifetime
    // we have two input lifetimes so rule 2 does NOT apply
    // there is a reference to self so rule 3 applies
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcept { 
        part: first_sentence 
    };
}