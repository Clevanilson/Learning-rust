
use std::fs::File;
use std::io::{ErrorKind, Error, Read};

fn main() {
    // panic!("Crash and Burn!") Unrecoverable error with panic! macro. Throws a new error.

    /* Recoverable errors with the Result Struct.
        Result<T, E> {
            Ok(T),
            Err(E),
        }
    */

    let file = File::open("hello.txt"); // Returns a Result type.

    let _file = match file {
        // if ok return the file.
        Ok(f) => f, 
        // if not ok, check the error type.
        Err(error) => match error.kind() {  
            // if error type is file not found, try to create a file.
            ErrorKind::NotFound => match File::create("hello.txt") { 
                // if ok return the file.
                Ok(fc) => fc,
                // if error on creating a file, panic and terminate.
                Err(error_2) => panic!("Problem creating the file {:?}", error_2),
            },
            // if other error type, panic and terminate.
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        }
    };
    
    
    // Panic on error.
    // File::open("hello_2.txt").unwrap();
    // File::open("hello_2.txt.").expect("message");
    
    // Propagating error
    println!("{:?}", read_from_file().unwrap());
    println!("{:?}", read_from_file_2().unwrap());

}

fn read_from_file() -> Result<String, Error> {
  let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}

fn read_from_file_2() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?; // if Err() return it;
    Ok(s)
}
