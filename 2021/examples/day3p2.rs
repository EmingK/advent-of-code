use std::io::{self, BufRead};

enum SearchDirection {
    Lower,
    Upper,
}

fn search(values: &[&[u8]], criteria: impl Fn(usize, usize) -> SearchDirection) -> i32 {
    let mut search_range = &values[..];
    let len = values[0].len();
    for idx in 0..len {
        if search_range.len() == 1 {
            break
        }
        if let Some(pos) = search_range.iter().position(|item| { item[idx] == b'1' }) {
            match criteria(search_range.len(), pos) {
                SearchDirection::Lower => search_range = &search_range[..pos],
                SearchDirection::Upper => search_range = &search_range[pos..],
            }
        }
    }
    search_range[0].iter().fold(0, |res, val| { res * 2 + i32::from(val - b'0') } )
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut lines = stdin_lock.lines().map(|line| { line.unwrap() }).collect::<Vec<_>>();
    lines.sort_unstable();
    let data = lines.iter().map(|line| { line.as_bytes() }).collect::<Vec<_>>();

    let oxygen = search(&data[..], |size, idx| {
        if idx <= (size / 2) {
            return SearchDirection::Upper;
        }
        return SearchDirection::Lower;
    });
    let co2 = search(&data[..], |size, idx| {
        if idx <= (size / 2) {
            return SearchDirection::Lower;
        }
        return SearchDirection::Upper;
    });
    println!("{}", oxygen * co2);
}