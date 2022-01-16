// Bring libraries to the scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop { // Creates an infinite loop
        println!("Please, enter your guess.");
        
        let mut guess = String::new(); // mut defines a mutable variable.

        io::stdin()
            .read_line(&mut guess) // & indicates that this argument is a reference.
            .expect("Fail to read line"); // Handling potential failures.

        // Shadowing previous guess value, and reuse the same variable name.
        let guess: u32 = guess.trim().parse().expect("Please, enter a number");

        println!("You guessed: {}", guess); // {} value placeholder 

        match guess.cmp(&secret_number) {
            Ordering::Equal => { 
                println!("You Win!");
                break;
            },
            Ordering::Greater => print!("Too big... "),
            Ordering::Less => print!("Too small... "),
        }
    }
}
