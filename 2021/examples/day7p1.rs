use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut lines = stdin_lock.lines().map(|line| { line.unwrap() });

    let input = lines.next().unwrap();
    let mut data = input.split(',').map(|s| { s.parse::<i32>().unwrap() }).collect::<Vec<_>>();

    data.sort_unstable();
    let target = data[data.len() / 2];
    let mut result = 0;
    for n in data {
        result += i32::abs(target - n);
    }
    println!("{}", result);
}