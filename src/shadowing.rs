#[allow(dead_code)]
pub(crate) fn main() {
    // const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    // println!("{} seconds in three hours", THREE_HOURS_IN_SECONDS);

    let x = 5; // initial declaration

    let x = x + 1; //new shadow declaration of x, x = 6

    {
        let x = x * 2; // new scoped shadow declaration of x, x = 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // outer scope, x = 6
}
