use std::io::{self, BufRead};

fn main() {
    let mut last: Option<i32> = None;
    let mut count = 0i32;
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    stdin_lock.lines().map(|line| {
        line.unwrap().parse::<i32>().unwrap()
    }).for_each(|val| {
        if let Some(last_val) = last {
            if val > last_val {
                count = count + 1;
            }
        }
        last = Some(val);
    });
    println!("{}", count);
}