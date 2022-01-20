// Defining a module in the same file.
mod greetings {
    pub fn hello() {
        println!("Hello!");
    }

    pub fn hi() {
        println!("Hi!");
    }

    pub fn howdy() {
        println!("Howdy!");
    }

    pub fn how_are_you() {
        println!("How are you?");
    }

    pub fn how_are_ya() {
        println!("How are ya?")
    }
}

// Importing function from module in the global scope.
use greetings::hi;
// Renaming Imports 
use greetings::howdy as how;
// Multiple imports
use greetings::{how_are_you, how_are_ya as hay};

// Defining module from other file.
mod thanks_responses;

use thanks_responses::no_problem;

//  Defining module from other directory
mod math;

fn main() {
    greetings::hello();
    hi();
    how();
    how_are_you();
    hay();

    thanks_responses::you_are_welcome();
    no_problem();

    println!("Sum: {}", math::sum(10, 10));
}
