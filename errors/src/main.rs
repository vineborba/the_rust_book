use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    first();

    let file = File::open("hello.txt");

    // like callback hell, but different
    let file = match file {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            other_error => {
                panic!("Can't open file: {:?}", other_error)
            }
        },
    };

    // clojures, chapter 13!
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Can't create file: {:?}", error))
        } else {
            panic!("Can't open file: {:?}", error)
        }
    });

    let file = File::open("hello.txt").unwrap(); // -> same as expect, but without message
    let file = File::open("hello.txt").expect("Can't open file: hello.txt");

    // same as
    // let file = File::open("hello.txt");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => panic!("Can't open file: {:?}", error),
    // };

    let file = File::open("hello.txt")?; // -> only if change return main type
    Ok(())

}

fn read_username_from_file_v1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    // same as
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)

    // same as
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn first() {
    second();
}

fn second() {
    third(0)
}

fn third(num: i32) {
    if num == 13 {
        panic!("bad number"); // run with backtrace flag for details
    }
}
