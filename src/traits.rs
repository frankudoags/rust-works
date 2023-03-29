// We define a trait called Summary.
// We can then implement this trait for any type we want.
// We can then call the summarize method on any type that implements the Summary trait.
// This is a very powerful feature of Rust.
// It allows us to define a behavior that types can implement,
// somewhat like an interface in other languages.

/// We can define a trait using the trait keyword
/// We can then define the behavior that types that implement this trait will need to have
/// We can then implement this trait for any type we want
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

pub trait TweetSummary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl TweetSummary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}