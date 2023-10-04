pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// this will use a default implementation
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// can use default implementations to define behavior in traits
// to do this, we can add a method body to say that our summary trait
// has a method called summarize that returns a Read more string
// a custom implementation of the trait can override the default
// can also call other methods within the trait implementation 
// we can use more than one summarization function
pub trait Summary {
    // this does not have a default implementation
    fn summarize_author(&self) -> String;

    // this has a default implementation and uses the summarize_author method
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// here we return any type that implements the Summary trait
// inside the function body, we return a Tweet
// returning without a concrete type is useful inside of closures and iterators
// impl return type has a restriction where if we were to return based on a boolean,
// this would not be allowed - we can only return one type 
// this is due to how the compiler impliments the impl syntax
fn returns_summarizable() -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("This is a headline"),
            author: String::from("This Author"),
            content: String::from("This is the content"),
        }
    } else {
        Tweet {
            username: String::from("@user2"),
            content: String::from("This is a tweet"),
            reply: false,
            retweet: false,
        }
    }
}

// we have this new function for notification that takes in a reference
// to something that implements Summary
// the print calls summarize() on the item
// impl works for straightforward cases but is actually syntax for a trait bound
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// what about a scenario where trait bounds give us more power?
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// if we wanted both item1 and item2 to be exactly the same type?
// can express that with trait bound syntax but not with impl syntax
// this says that they must both be of type T
pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// specifying multiple trait bounds could hinder readabiltiy of code
// there is a lot of text between function name and parameters
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// we can fix the above using the where clause
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}

fn main() {
    // we have two structs that have different fields
    // what if we wanted to summarize a news article or tweet?
    // we can use a trait to define shared behavior between the two structs
    // shared behaviors means methods - traits will allow us to define methods
    // that are shared between different types
    let tweet = Tweet {
        username: String::from("@user1"),
        content: String::from("This is a tweet"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("This Author"),
        headline: String::from("This is a headline"),
        content: String::from("This is the content"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());

    // what about traits as parameters?
    // we can use impl Trait syntax to accept any type that implements a trait
    notify(&article);

    // can use the returns_summarizable method to return a Tweet
    println!("New tweet available! {}", returns_summarizable().summarize());
}