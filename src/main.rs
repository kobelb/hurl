use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
struct Command {
  method: String,
  path: String
}

impl fmt::Display for Command {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "method: {}, path: {}", self.method, self.path)
  }
}

fn main() {
  loop {
    match read() {
      Ok(c) => { evaluate(c) }
      Err(err) => { println!("Unable to parse command: {}", err) }
    }
  }
}

fn read() -> Result<Command, Box<Error>> {
  let mut buffer = String::new();
   try!(io::stdin().read_line(&mut buffer));
   parse(buffer)
}

fn parse(text: String) -> Result<Command, Box<Error>> {
  let mut iter = text.split_whitespace();

  Ok(
    Command {
      method: iter.next().unwrap().to_string(),
      path: iter.next().unwrap().to_string()
    }
  )
}

fn evaluate(command: Command) {
  println!("Gonna execute: {}", command);
}