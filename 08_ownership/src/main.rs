fn main() {
    /* 
        Ownership is a set of rules that governs how Rust manage memory
        Each value in Rust has a variable that’s called its owner.
        There can only be one owner at a time.
        When the owner goes out of scope, the value will be dropped.
    */

    let x= 1; // Binds the value 1 to x.
    let y= x; // Make a copy of the value in x.
    //  This happens because integers has a known size.

    println!("x: {} y: {}", x, y);

    let string_1 = String::from("Hello"); // Binds the value "Hello" to string_1.
    let string_2 = string_1; // Move value from string_1 to string_2. string_1 is not valid anymore.
    let string_3 = string_2.clone(); // Clone the value from string 2 on the heap.

    println!("string_1: not valid string_2: {} string_3: {}", string_2, string_3);

    // Ownership and functions.

    let string_4 = String::from("Hi, there"); // string_4 comes into scope.

    take_ownership(string_4); // function takes ownership of string_4 value.
    // string_4 is not valid anymore.

    let z = 2; // z comes into scope.

    make_copy(z); // z value is copied by the function.

    let string_5 = gives_ownership(); // Gets ownership from this function's return.

    println!("{}", string_5);
}


fn take_ownership(some_string: String) {
    println!("Some string: {}", some_string);
}

fn make_copy(some_integer: u8) {
    println!("Some integer: {}", some_integer);
}

fn gives_ownership() -> String {
    let string_1 = String::from("Gives Owneship");

    string_1
}