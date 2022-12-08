use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let lines = stdin_lock.lines().map(|line| { line.unwrap() });

    let result = lines.fold(0, |res, line| {
        let count = line.split('|').nth(1).unwrap().trim().split(' ').filter(|s| {
            match s.len() {
                2|3|4|7 => true,
                _ => false
            }
        }).count();
        res + count
    });
    println!("{}", result);
}