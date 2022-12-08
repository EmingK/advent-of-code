use std::io::{self, BufRead};

fn translate_buf(buf: impl Iterator<Item = u8>) -> impl Iterator<Item = i32> {
    buf.map(|val| {
        match val {
            b'1' => 1,
            _ => -1
        }
    })
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut lines = stdin_lock.lines();
    let mut vals = translate_buf(lines.next().unwrap().unwrap().bytes()).collect::<Vec<_>>();

    lines.for_each(|line| {
        vals.iter_mut().zip(translate_buf(line.unwrap().bytes())).for_each(|(v1, v2)| {
            *v1 += v2;
        });
    });
    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in vals {
        gamma *= 2;
        epsilon *= 2;
        if bit >= 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    println!("{}", gamma * epsilon);
}