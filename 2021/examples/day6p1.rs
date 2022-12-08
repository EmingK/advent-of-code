use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut lines = stdin_lock.lines().map(|line| { line.unwrap() });

    const CYCLE: usize = 7;
    const TIMER_MAX: usize = 8;
    const DAYS_MAX: usize = 80;

    let mut tbl_new = [1i32; DAYS_MAX + 1];
    for day in (TIMER_MAX + 1)..=DAYS_MAX {
        let total = day - TIMER_MAX;
        let mut current = (total - 1) % CYCLE;
        while current < total {
            tbl_new[day] += tbl_new[current];
            current += CYCLE;
        }
    }

    let mut tbl_kind = [1i32; CYCLE];
    for kind in 0..CYCLE {
        let total = DAYS_MAX - kind;
        let mut current = (total - 1) % CYCLE;
        while current < total {
            tbl_kind[kind] += tbl_new[current];
            current += CYCLE;
        }
    }

    let input = lines.next().unwrap();
    let data = input.split(',').map(|s| { s.parse::<usize>().unwrap() });
    let mut result = 0;
    for n in data {
        result += tbl_kind[n];
    }
    println!("{}", result);
}