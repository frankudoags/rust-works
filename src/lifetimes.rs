use std::fmt::Display;

pub(crate) fn _main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = largest(s1.as_str(), s2);

    println!("The largest string is {}", result);

    struct ImportantExcerpt<'a> {
        _part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn _level(&self) -> i32 {
            3
        }

        fn _announce_and_return_part(&self, anouncement: &str) -> &str {
            println!("Attention please: {}", anouncement);
            self._part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a'.'");

    let _i = ImportantExcerpt {
        _part: first_sentence,
    };
}

fn largest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
/// The lifetime of the return value is the same as the smaller of the lifetimes of the arguments.
/// This is called the lifetime elision rule.
/// The elision rules don't provide full inference: if Rust can't figure out what the lifetime of the
/// return value should be, the compiler will stop and make you use explicit lifetime annotations.
/// We pass in an extra generic type T, which we use to print the announcement.
/// We use the Display trait to print the announcement, which means we can use any type that implements
/// the Display trait.
/// Because lifetimes are a type of generic, the declaration of the largest_with_an_announcement
/// function is actually a generic function and contains two generic type parameters: the lifetime
/// parameter 'a and the generic type parameter T.
/// The Display trait bound on T means that any type used in the announcement parameter must implement
/// the Display trait.

fn _largest_with_an_announcement<'a, T>(a: &'a str, b: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
