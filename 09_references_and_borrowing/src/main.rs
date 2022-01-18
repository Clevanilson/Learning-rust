fn main() {
    let mut string_1 = String::from("Hello, World."); 
    let length = get_length(&string_1); // Takes a reference to the value on memory.
    
    println!("Value: {}, Length: {}", string_1, length); // Value of string_1 still available.

    mutate(&mut string_1); // Passing a mutable reference.

    println!("Changed value: {}", string_1);

    // you can have only one mutable reference to a particular piece of data at a time.
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // -> Error;
}

fn get_length(string: &String) -> usize {
    string.len()
}

fn mutate(string: &mut String) {
    string.push_str(" Today is a lovely day.");
}