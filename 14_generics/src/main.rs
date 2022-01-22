mod first_implementation;
mod second_implementation;
mod third_implementation;

fn main() {
    // Using generic types to prevent code repetition.
    first_implementation::run();
    second_implementation::run();
    third_implementation::run();
}

