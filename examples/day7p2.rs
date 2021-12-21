use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut lines = stdin_lock.lines().map(|line| { line.unwrap() });

    let input = lines.next().unwrap();
    let data = input.split(',').map(|s| { s.parse::<i32>().unwrap() }).collect::<Vec<_>>();

    let avg = (data.iter().fold(0, |a, b| { a + b })) as f32 / data.len() as f32;

    let target1 = avg.floor() as i32;
    let target2 = avg.ceil() as i32;

    let result1 = data.iter().fold(0, |res, n| {
        let distance = i32::abs(target1 - n);
        res + (distance * (distance + 1) / 2)
    });
    let result2 = data.iter().fold(0, |res, n| {
        let distance = i32::abs(target2 - n);
        res + (distance * (distance + 1) / 2)
    });

    println!("{}", i32::min(result1, result2));
}