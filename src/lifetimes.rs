pub(crate) fn _main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = largest(s1.as_str(), s2); 
    println!("The largest string is {}", result);
}

fn largest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}