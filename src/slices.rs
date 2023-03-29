#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let mut s1 = String::from("hello");
    let len = calc_length(&s1);
    
    change(&mut s1);
    println!("The length of '{}' is {}.", s1, len);

    let word = first_word(&s1[..]);
    println!("The first word is: {}", word);

}

fn calc_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    //converts string to array of bytes
    let bytes = s.as_bytes();
    // iterates over array of bytes,
    // we search for the byte that represents the space by using the byte literal syntax.
    // If we find a space, we return the position.
    // Otherwise, we return the length of the string by using s.len().
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
