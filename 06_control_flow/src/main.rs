use std::io;

fn main() {
    run_conditional();    
    run_loop();
}

fn run_conditional() {
    let mut value = String::new();

    println!("Enter a value");

    io::stdin().read_line(&mut value).expect("Fail to read line");

    let value: i32 = value.trim().parse().expect("Please, enter a number");

    if value > 5 {
        println!("Value greater than 5");
    }
    else if value < 5 {
        println!("Value less than 5");
    }

    let value = if value == 5 { "Equal" } else { "Not Equal "};

    println!("Value: {}", value);
}

fn run_loop() {
    // Loop
    let mut value = 0;

    loop {
        value += 1;

        println!("Counting loop {}", value);
        if value == 5 {
            break;
        }
    }

    // While
    let mut value = 0;

    while value < 5 {
        println!("Counting while {}", value);
        value += 1;
    }

    // For in 
    let array = [1, 2, 3];

    for element in array {
        println!("Element {}", element);
    }

    for number in 1..4 {
        println!("Range {}", number);
    }
}