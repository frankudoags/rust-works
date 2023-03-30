use crate::traits::{NewsArticle, Summary, Tweet, TweetSummary};
use std::fmt::Display;
#[allow(dead_code)]

pub(crate) fn _main() {
    let tweet: Tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let news: NewsArticle = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", news.summarize());

    struct TweetNoSummary {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    let _tweet_no_summary: TweetNoSummary = TweetNoSummary {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    //notify(&tweet_no_summary); // error[E0277]: the trait bound `Tweet_NoSummary: Summary` is not satisfied

    //notify(&tweet); // error[E0277]: the trait bound `Tweet: Summary` is not satisfied

    notify(&news); // ok, since NewsArticle implements Summary
}

// Base notify: takes an argument item, that must implement the Summary Trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Same as above, but with a generic type parameter, T which implements Summary,
// and item which is a reference to a value of type T,
/// This is the same as the above, but with a generic type parameter
pub fn gen_notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn gen_notify_multiple<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

//Specifying Multiple Trait Bounds with the + Syntax

pub fn gen_notify_multiple_plus<T: Summary + Display>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Clearer Trait Bounds with where Clauses

pub fn gen_notify_multiple_plus_where<T, U>(item1: &T, item2: &U)
where
    T: Summary + Display,
    U: Summary + Display,
{
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

/// Using Trait Bounds to Conditionally Implement Methods
/// the cmp_display method is only available to Pair<T> instances where T implements the PartialOrd trait
/// and the Display trait
struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
