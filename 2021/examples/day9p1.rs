use std::io::{self, BufRead};
use std::mem;

struct Line {
    last: Option<Vec<u8>>,
    this: Option<Vec<u8>>,
    next: Option<Vec<u8>>,
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut lines = stdin_lock.lines()
        .map(|line| { Some(line.unwrap().bytes().map(|d| d - b'0').collect::<Vec<_>>()) })
        .chain(std::iter::once(None));

    let mut processor = Line {
        last: None,
        this: None,
        next: lines.next().unwrap(),
    };

    let result = lines.fold(0, |res, line| {
        let mut tmp = mem::replace(&mut processor.next, line);
        mem::swap(&mut processor.this, &mut tmp);
        let _ = mem::replace(&mut processor.last, tmp);
        
        res + processor.this.as_ref().unwrap().iter().enumerate().fold(0i32, |res, (idx, &val)| {
            let top = processor.last.as_ref().and_then(|v| v.get(idx)).map(|&that| that > val).unwrap_or(true);
            let left = if idx > 0 {
                processor.this.as_ref().and_then(|v| v.get(idx - 1)).map(|&that| that > val).unwrap_or(true)
            } else {
                true
            };
            let right = processor.this.as_ref().and_then(|v| v.get(idx + 1)).map(|&that| that > val).unwrap_or(true);
            let bot = processor.next.as_ref().and_then(|v| v.get(idx)).map(|&that| that > val).unwrap_or(true);

            res + if top & left & right & bot {
                (val + 1) as i32
            } else {
                0
            }
        })
    });
    
    println!("{}", result);
}