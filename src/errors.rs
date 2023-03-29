use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Write},
};

pub(crate) fn _main() {
    // let f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => panic!("There was a problem opening the file: {:?}", other_error),
    //     },
    // };

    _cleaner_main();
}

pub fn _cleaner_main() {
    let mut f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");
    println!("\n\n\n{} \n\n\n", buffer);
}

pub fn read_and_write_main() {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("hello.txt")
        .expect("Failed to open file");

    f.write_all(b"Hello, world!")
        .expect("Failed to write to file");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");
    println!("{}", buffer);
}
