struct User {
    name: String,
    age: i8,
}

impl User {
  fn log(&self) {
    println!("Name: {} | Age: {}", self.name, self.age);
  }
}

impl  User {
  fn can_log(active: bool) {
    
    if active {
      println!("Can Log");
    }
    else {
      println!("Can't Log");
    }
  }
}

struct Tuple(i8);

fn main() {
  let user_1 = User {
      age: 1,
      name: String::from("User Name 1"),
  };

  let user_2 = build_user(String::from("User Name 2"), 2);

  let user_3  = User {
    name: String::from("User Name 3"),
    ..user_2
  };

  let tuele_struct = Tuple(1);

  user_1.log();
  user_2.log();
  user_3.log();
  User::can_log(true);
  User::can_log(false);
  println!("{}", tuele_struct.0);
}

fn build_user(name: String, age: i8) -> User {
  User { name, age }
}