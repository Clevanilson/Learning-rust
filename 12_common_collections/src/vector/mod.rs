pub fn run() {
  println!("---------------------------------------");
  println!("VECTORS");
  let mut vector_1: Vec<i32> = Vec::new(); // Creates a new vector.
  println!("{:?}", vector_1);
  vector_1.push(1); // Adds an new element to the vector.
  println!("{:?}", vector_1);
  let vector_2 = vec!(1, 2, 3); // Creates a new vector with values.
  println!("{:?}", vector_2);
  println!("Accessing values inside a vector: {}", vector_2[0]);

  match vector_2.get(2) { // Get returns a option type (value or none)
      Some(value) => println!("Has Value: {value}"),
      None => println!("No Value")
  }

  match vector_2.get(5) { 
      Some(value) => println!("Has Value: {value}"),
      None => println!("No Value")
  }

  for value in &mut vector_1 {
      *value *= 2;
  }

  println!("{:?}", vector_1);
}