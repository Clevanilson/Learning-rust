fn main() {
    print_hello_world();
    print_value(3);
    println!("Square of 2 is: {}", square(2));
}

fn square(number: i32) -> i32 {
    // return expressions does not have a semicolon. you can return earlier by usin the <return> keyword.
    number * number 
}

fn print_hello_world() {
    println!("Hello, World from function!");
}

fn print_value(number: i32) {
    println!("Value of x = {}", number);
}