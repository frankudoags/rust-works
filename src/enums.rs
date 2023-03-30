#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
#[allow(unused_mut)]

enum IPAddrKind {
    V4(String),
    V6(String),
}
pub(crate) fn _main() {
    let _home = IPAddrKind::V4(String::from("127.0.0.1"));
    let _work = IPAddrKind::V6(String::from("127.0.0.2"));

    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let penny = Coin::Penny;

    println!("The value of penny is {}", value_in_cents(penny));
}
