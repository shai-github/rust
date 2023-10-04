/*
There are instances where compiler can determinastically infer lifetime annotations.
It does this by checking the three lifetime elision rules. 

Note that lifetimes of parameters passed in are impl lifetimes and lifetimes of return
values are output lifetimes. The rules are:

1. Each parameter that is a reference gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self 
   is assigned to all output lifetime parameters. (this only applies to methods)

If compiler goes through rules and cannot identify the lifetime, then we will have to specify it.
*/

fn main() {
    println!("Hello, world!");
}

// you could remove the lifetime annotations here and the code would still compile
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    
    &s[..]
}