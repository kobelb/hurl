extern crate curl;
extern crate url;

use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use url::{Url};

use curl::easy::Easy;

#[derive(Debug)]
struct Command {
  method: String,
  path: String
}

struct ServerConfig {
  url: String,
  username: Option<String>,
  password: Option<String>
}

impl fmt::Display for Command {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "method: {}, path: {}", self.method, self.path)
  }
}

fn main() {
  let server_config = ServerConfig {
    url: "http://localhost:9200".to_string(),
    username: None,
    password: None
  };

  loop {
    match read() {
      Ok(command) => { evaluate(&server_config, &command) }
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

fn evaluate(server_config: &ServerConfig, command: &Command) {
  let mut easy = Easy::new();
  let url = Url::parse(&server_config.url).unwrap();
  let url = url.join(&command.path).unwrap();

  match server_config.username {
    Some(ref username) => { easy.username(username).unwrap() },
    None => {}
  }

  match server_config.password {
    Some(ref password) => { easy.password(password).unwrap() }
    None => {}
  }

  easy.custom_request(&command.method).unwrap();
  easy.url(url.as_str()).unwrap();
  easy.write_function(|data| {
      Ok(io::stdout().write(data).unwrap())
  }).unwrap();
  easy.perform().unwrap();

  println!("{}", easy.response_code().unwrap());
}