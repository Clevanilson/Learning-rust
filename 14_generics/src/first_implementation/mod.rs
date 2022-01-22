pub fn run() {
  // Basic implementation, repeating code.
  println!("---------------------------------------------------------");
  let vector_1 = vec!(1, 2, 3, 10, 20, 30, 99);
  let mut largest_1 = vector_1[0];

  for value in vector_1 {
    if value > largest_1 {
      largest_1 = value;
    }
  }

  println!("Largest vector_1 value: {}", largest_1);

  let vector_2 = vec!(4, 5, 6, 20, 30, 40, 199);
  let mut largest_2 = vector_2[0];

  for value in vector_2 {
    if value > largest_2 {
      largest_2 = value;
    }
  }

  println!("Largest vector_2 value: {}", largest_2);
}