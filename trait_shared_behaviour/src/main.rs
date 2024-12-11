// Traits: Defining Shared Behavior

// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
// example of a trait:
pub trait Summary {
    fn summarize(&self) -> String;
}

//Here, we declare a trait using the trait keyword and then the trait’s name, which is Summary in this case. 
// We also declare the trait as pub so that crates depending on this crate can make use of this trait too, as we’ll see in a few examples. 
// Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait, which in this case is fn summarize(&self) -> String.



//Implementing a Trait on a Type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//Implementing a trait on a type is similar to implementing regular methods. 
// The difference is that after impl, we put the trait name we want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for.


//Now that the library has implemented the Summary trait on NewsArticle and Tweet, users of the crate can call the trait methods on instances of NewsArticle and Tweet in the same way we call regular methods. 
// The only difference is that the user must bring the trait into scope as well as the types. Here’s an example of how a binary crate could use our aggregator library crate:

// use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

