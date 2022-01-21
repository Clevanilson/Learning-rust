pub fn run() {
  println!("---------------------------------------");
  println!("STRINGS");
  let mut string_1 = String::new(); // Creates an empty string.
  let string_2 = "Slice"; // Creates an string Slice.
  let string_3 = string_2.to_string(); // Creates an string from a string slice.
  let string_4 = String::from("New String"); // Creates an string.

  string_1.push_str("Hello, World"); // Append string.
  string_1.push('!'); // Append char.

  println!("{string_1} | {string_2} | {string_3} | {string_4}");
  // Append using the + operator.
  let string_1 = String::from("Hello, ");
  let string_2 = String::from("World!");

  let string_1 = string_1 + &string_2;

  println!("{string_1} | {string_2}");

}