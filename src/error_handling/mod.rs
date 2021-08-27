use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{io, fs};

#[allow(dead_code)]
pub fn execute() {
    recoverable_errors_with_result();
    matching_on_different_errors();
    propagating_errors();
}

fn recoverable_errors_with_result() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(error) => panic!("Error opening the file: {:?}", error),
    };
}

fn matching_on_different_errors() {
    let f = File::open("hello.txt");

    // with nested match expressions
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error creating the file: {:?}", error),
            },
            other_error => panic!("Error opening the file: {:?}", other_error),
        },
    };

    // with unwrap_or_else
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating the file: {:?}", error);
            })
        } else {
            panic!("Error opening the file: {:?}", error);
        }
    });
}

fn propagating_errors() {
    match read_username_from_file() {
        Ok(username) => println!("username: {}", username),
        Err(error) => panic!("{}", error),
    }
}

/// A function that returns errors to the calling code.
fn read_username_from_file() -> Result<String, io::Error> {
    // read_username_from_file_v1()
    // read_username_from_file_v2()
    // read_username_from_file_v3()
    read_username_from_file_v4()
}

#[allow(dead_code)]
fn read_username_from_file_v1() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}
