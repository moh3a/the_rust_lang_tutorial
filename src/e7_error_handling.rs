use std::{
    fs::{self, File},
    io::Error,
    // io::{Error, ErrorKind, Read},
};

#[allow(dead_code)]
pub fn run() {
    // panic!("BURRRRN!"); // ! panic! marco to exit process

    // let f = File::open("hello.txt");
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file.\n{:?}", e),
    //         },
    //         other_error => panic!("Probem opening the file.\n{:?}", other_error),
    //     },
    // };

    // the commented out code above is correct but instead of using the match expression, we use the unwrap()
    let _f = File::open("hello.txt").expect("failed to open file.");
}

#[allow(dead_code)]
fn read_username_from_file() -> Result<String, Error> {
    // let mut s = String::new();

    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // we can also write it this way
    // let mut f = File::open("hello.txt")?;

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => return Err(e),
    // }
    // also simplify the above with
    // f.read_to_string(&mut s)?;

    // simplify even further to
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // write everything in one line
    fs::read_to_string("hello.txt")
}
