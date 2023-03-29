use rand::Rng;
use std::cmp::Ordering;
use std::io;
#[allow(dead_code)] // This is to prevent the compiler from complaining about the main function not being used

pub(crate) fn main() {
    //Title
    println!("GUESS THE NUMBER GAME!\n\n\n");
    // Generate secret number from rand Library
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //Flag to trigger additional warning to prompt
    let mut valid = false;
    // Loop to read user guess and compare with secret number
    // Loops until user gets secret number, then breaks.
    loop {
        //check trigger to know if prompt should include additional warning
        if !valid {
            println!("Please input your guess:");
        } else {
            println!("Please input your guess (1-100):");
        }
        // Variable to store user input
        let mut guess = String::new();
        //Reads users input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess");
        //Converts user input to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                valid = true;
                continue;
            }
        };
        //Compares user input to secret number
        //Prints out if user input is too big or too small
        //Breaks loop if user input is equal to secret number
        //Prints out secret number if user wins
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!, the secret number is {}", secret_number);
                break;
            }
        }
    }
}
