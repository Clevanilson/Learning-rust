use std::{ fs, error::Error };

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(config.file_path)?;

  for line in search(&config.query, &content) {
    println!("{line}");
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_path: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 3 {
          return Err("Missing arguments");
      }

       let query = args[1].clone();
       let file_path = args[2].clone();

      Ok(Config { query, file_path })
  }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for lines in content.lines() {
    if lines.contains(query) {
      results.push(lines);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "holl";
    let content = 
      "But I, I know
That you're long gone
But I, I will
Go on, howling and hollow
To never know";

    assert_eq!(vec!("Go on, howling and hollow"), search(query, content));

  }


}