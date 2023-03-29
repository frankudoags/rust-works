#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
#[allow(unused_mut)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct _UserOwnership<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

pub(crate) fn _main() {
    let mut mide: User = User {
        active: true,
        username: String::from("butterscotch"),
        email: String::from("fateye@gmail.com"),
        sign_in_count: 1,
    };
    // Since `mide` is mutable, we can change the email property
    mide.email = String::from("fateye001@gmail.com");
    println!("Mide details are:\n {:#?}", mide);

    let mut _alrho: User = User {
        email: String::from("ridwan007@gmail.com"),
        username: String::from("Alrho"),
        ..mide
    };
    let mut _bosun = build_user(String::from("bosun-ajb@gmail.com"), String::from("Bosun"));


    fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
}


// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
