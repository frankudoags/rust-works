#[allow(dead_code)]
#[allow(unused_variables)]
pub(crate) fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let len = array.len();
    
    let slice: &[i32] = &array[0..3]; //valid
    let a = &array[..]; //valid
    let b = &array[0..len]; //valid

    println!("slice: {:?}", slice);




    let v = true;
    let tenary = if v { true } else { false };

    let mut index = 0;

    println!("\nWhile loop");
    while index < 5 {
        println!("the value is: {}", array[index]);

        index += 1;
    }

    println!("\nFor...in loop");
    for elem in array {
        println!("the value is: {elem}");
    }

    
}
