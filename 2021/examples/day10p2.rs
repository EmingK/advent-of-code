use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let lines = stdin_lock.lines().map(|line| { line.unwrap() });

    let mut scores = lines.filter_map(|line| {
        let mut stack = Vec::new();
        let mut score = 0i64;
        for char in line.bytes() {
            match char {
                b'(' | b'[' | b'{' | b'<' => stack.push(char),
                b')' | b']' | b'}' | b'>' => {
                    let top = stack.pop();
                    if top != Some(char - 1) && top != Some(char - 2) {
                        return None
                    }
                },
                _ => (),
            }
        }
        while let Some(char) = stack.pop() {
            score *= 5;
            score += match char {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => 0,
            };
        }
        Some(score)
    }).collect::<Vec<_>>();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}