use std::io::{self, BufRead};
use std::num::ParseIntError;
//use std::option::NoneError;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    ParseError,
}

// impl From<NoneError> for Error {
//     fn from(_: NoneError) -> Self {
//         Self::ParseError
//     }
// }

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::ParseError
    }
}

enum CommandType {
    Forward,
    Down,
    Up
}

impl FromStr for CommandType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => return Err(Error::ParseError)
        };
        Ok(result)
    }
}

struct Command {
    cmd_type: CommandType,
    value: i32
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let cmd_type: CommandType = parts.next().ok_or(Error::ParseError)?.parse()?;
        let value: i32 = parts.next().ok_or(Error::ParseError)?.parse()?;

        Ok(Self {
            cmd_type,
            value
        })
    }
}

fn main() {
    let mut pos = (0i32, 0i32);
    
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    stdin_lock.lines().map(|line| {
        line.unwrap().parse::<Command>().unwrap()
    }).for_each(|val| {
        let Command {cmd_type, value} = val;
        let multiplier = match cmd_type {
            CommandType::Forward => (1, 0),
            CommandType::Up => (0, -1),
            CommandType::Down => (0, 1),
        };
        pos.0 += value * multiplier.0;
        pos.1 += value * multiplier.1;
    });
    println!("{}", pos.0 * pos.1);
}