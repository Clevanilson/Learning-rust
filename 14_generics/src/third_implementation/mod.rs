pub fn run() {
  // Function implementation.
  println!("---------------------------------------------------------");
  let vector_1 = vec!(1, 2, 3, 10, 20, 30, 99);
  let vector_2 = vec!(4, 5, 6, 20, 30, 40, 199);
  let vector_3 = vec!('a', 'b', 'c');

  println!("Largest vector_1 value: {}", get_largest(vector_1));
  println!("Largest vector_2 value: {}", get_largest(vector_2));
  println!("Largest vector_3 value: {}", get_largest_char(vector_3));

}

// This function implementation resolves the repetition problem
//  with vectors of the same type.
fn get_largest(vector: Vec<i32>) -> i32 {
  let mut largest = vector[0];

  for value in vector {
    if value > largest {
      largest = value;
    }
  }

  largest
}

// Requires other function to handle other data type.
fn get_largest_char(vector: Vec<char>) -> char {
  let mut largest = vector[0];

  for value in vector {
    if value > largest {
      largest = value;
    }
  }

  largest
}