use std::io::{self, BufRead};

fn main() {
    const BUF_SIZE: usize = 3;

    let mut buf = [0i32; BUF_SIZE];
    let mut count = 0i32;
    let mut idx = 0;
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut data = stdin_lock.lines().map(|line| {
        line.unwrap().parse::<i32>().unwrap()
    });
    (0..BUF_SIZE).for_each(|idx| {
        buf[idx] = data.next().unwrap();
    });
    data.for_each(|val| {
        if val > buf[idx] {
            count = count + 1;
        }
        buf[idx] = val;
        idx = (idx + 1) % BUF_SIZE;
    });
    println!("{}", count);
}