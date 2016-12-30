extern crate curl;
extern crate url;

use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use url::{Url, ParseError};

use curl::easy::Easy;

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
  let mut easy = Easy::new();
  let url = Url::parse("http://localhost:9200").unwrap();
  let url = url.join(&command.path).unwrap();
  easy.url(url.as_str()).unwrap();
  easy.write_function(|data| {
      Ok(io::stdout().write(data).unwrap())
  }).unwrap();
  easy.perform().unwrap();

  println!("{}", easy.response_code().unwrap());
}