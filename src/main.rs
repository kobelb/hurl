extern crate curl;
extern crate termion;
extern crate url;

use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use url::Url;

use curl::easy::Easy;

#[derive(Debug)]
struct Command {
    method: String,
    path: String,
}

struct ServerConfig {
    url: String,
    username: Option<String>,
    password: Option<String>,
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
        password: None,
    };

    // loop {
    match read() {
        Ok(command) => evaluate(&server_config, &command),
        Err(err) => println!("Unable to parse command: {}", err),
    }
    // }
}

fn read() -> Result<Command, Box<Error>> {
    let stdin = io::stdin();
    let mut stdout = try!(io::stdout().into_raw_mode());
    try!(write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)));
    try!(stdout.flush());


    let mut lines: Vec<u16> = vec![0];
    let mut line: u16 = 0;
    let mut column: u16 = 0;
    let mut buffer = String::new();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::F(5) => break,
            Key::Char(c) => {
                try!(write!(stdout, "{}", c));
                buffer.push(c);
                
                if c == '\n' {
                    line += 1;
                    lines.insert(line as usize, 0);
                    column = 0;
                } else {
                    lines[line as usize] += 1;
                    column += 1;
                }
            }
            Key::Left => {
                if column > 0 {
                    column -= 1;
                }
            },
            Key::Right => {
                if column < lines[line as usize] {
                    column += 1;
                }
            },
            Key::Up => {
                if line > 0 {
                    line -= 1;
                }
            },
            Key::Down => {
                if (line as usize) < lines.len() - 1 {
                    line += 1;
                }
                if column > lines[line as usize] {
                    column = lines[line as usize];
                }
            },
            Key::Ctrl('c') => panic!("ciao"),
            _ => println!("Other"),
        }
        try!(write!(stdout, "{}", cursor::Goto(column + 1, line + 1)));
        try!(stdout.flush());
    }

    parse(buffer)
}

fn parse(text: String) -> Result<Command, Box<Error>> {
    let mut iter = text.split_whitespace();

    Ok(Command {
        method: iter.next().unwrap().to_string(),
        path: iter.next().unwrap().to_string(),
    })
}

fn evaluate(server_config: &ServerConfig, command: &Command) {
    let mut easy = Easy::new();
    let url = Url::parse(&server_config.url).unwrap();
    let url = url.join(&command.path).unwrap();

    match server_config.username {
        Some(ref username) => easy.username(username).unwrap(),
        None => {}
    }

    match server_config.password {
        Some(ref password) => easy.password(password).unwrap(),
        None => {}
    }

    easy.custom_request(&command.method).unwrap();
    easy.url(url.as_str()).unwrap();
    easy.write_function(|data| Ok(io::stdout().write(data).unwrap()))
        .unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
