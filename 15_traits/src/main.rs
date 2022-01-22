pub struct Client {
    name: String,
}

impl Greet for Client {
    fn greet(&self) {
        println!("Hello, {}", self.name);
    }

    fn default_greet(&self) {
        println!("Default implementation was override by: {}", self.name);
    }
}

pub struct User {
    name: String,
}

impl Greet for User {
    fn greet(&self) {
        println!("Hello {}", self.name);
    }
}

pub trait Greet {
    // Define a method to be implemented.
    fn greet(&self);

    // Define a method to be implemented or use the default implementation.
    fn default_greet(&self) {
        println!("Default Implementation of default_greet!");
    }
}

// Traits as functions arguments.
// Takes a reference of something that implements the Greet Trait.
pub fn greet(arg: &impl Greet) {
    arg.greet();
}

// or 
/*
pub fn greet<T: Greet>(arg: &T) {
    arg.greet();
}
*/



fn main() {
    // Traits defines a shared behavior (methods).
    let client = Client {
        name: String::from("Client"),
    };

    let user = User {
        name: String::from("User"),
    };

    client.greet();   
    client.default_greet();
    user.greet();
    user.default_greet();

    greet(&client);
}
