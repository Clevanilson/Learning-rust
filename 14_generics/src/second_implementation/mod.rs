pub fn run() {
  // Function implementation with generics.
  println!("---------------------------------------------------------");
  let vector_1 = vec!(1, 2, 3, 10, 20, 30, 99);
  let vector_2 = vec!(4, 5, 6, 20, 30, 40, 199);
  let vector_3 = vec!('a', 'b', 'c');

  println!("Largest vector_1 value: {}", get_largest(vector_1));
  println!("Largest vector_2 value: {}", get_largest(vector_2));
  println!("Largest vector_3 value: {}", get_largest(vector_3));

}

// <T> notation specify the function to use generics.
// The function now takes a vector of T, and return a generic type T as well.
// The PartialOrd trait tells the T must be an type that can be ordered,
// and Copy tells the T must be a type that could be copied.
fn get_largest<T: PartialOrd + Copy>(vector: Vec<T>) -> T {
  let mut largest = vector[0];

  for value in vector {
    if value > largest {
      largest = value;
    }
  }

  largest
}
