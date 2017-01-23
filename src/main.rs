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

struct Position {
    line: usize,
    column: usize
}

fn read() -> Result<Command, Box<Error>> {
    let stdin = io::stdin();
    let mut stdout = try!(io::stdout().into_raw_mode());
    try!(write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)));
    try!(stdout.flush());


    let mut lines: Vec<String> = vec![String::new()];
    let mut position = Position{line: 0, column: 0};
    
    for c in stdin.keys() {
        match c.unwrap() {
            Key::F(5) => break,
            Key::Char(c) => {
                let trailing: String = lines[position.line][position.column..].to_string();
                if c == '\n' {
                    lines[position.line].truncate(position.column);
                    position.line += 1; 
                    position.column = 0;
                    try!(write!(stdout, "{}{}{}{}", clear::AfterCursor, c, cursor::Goto((position.column as u16) + 1, (position.line as u16) + 1), trailing));
                    lines.insert(position.line, trailing);
                } else {
                    try!(write!(stdout, "{}{}{}", clear::AfterCursor, c, trailing));
                    lines[position.line].insert(position.column, c);
                    position.column += 1;
                }
            },
            Key::Left => {
                if position.column > 0 {
                    position.column -= 1;
                }
            },
            Key::Right => {
                if position.column < lines[position.line].len() {
                    position.column += 1;
                }
            },
            Key::Up => {
                if position.line > 0 {
                    position.line -= 1;
                }
            },
            Key::Down => {
                if (position.line) < lines.len() - 1 {
                    position.line += 1;

                    let line_len = lines[position.line].len();
                    if (position.column) > line_len {
                        position.column = line_len;
                    }
                }
            },
            Key::Ctrl('c') => panic!("ciao"),
            _ => {},
        }
        
        try!(write!(stdout, "{}", cursor::Goto((position.column as u16) + 1, (position.line as u16) + 1)));
        try!(stdout.flush());
    }
    let text = lines.join("\n");
    try!(write!(stdout, "{}{}", cursor::Goto((position.column as u16) + 2, (position.line as u16) + 1), text));
    parse(text)
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
