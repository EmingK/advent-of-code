use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let lines = stdin_lock.lines().map(|line| { line.unwrap() });

    let result = lines.fold(0, |res, line| {
        let mut stack = Vec::new();
        let mut score = 0;
        for char in line.bytes() {
            match char {
                b'(' | b'[' | b'{' | b'<' => stack.push(char),
                b')' => {
                    if stack.pop() != Some(b'(') {
                        score = 3;
                    }
                },
                b']' => {
                    if stack.pop() != Some(b'[') {
                        score = 57;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        score = 1197;
                    }
                },
                b'>' => {
                    if stack.pop() != Some(b'<') {
                        score = 25137;
                    }
                },
                _ => (),
            }
        }
        res + score
    });
    println!("{}", result);
}