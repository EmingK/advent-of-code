use std::collections::LinkedList;
use std::io::{self, BufRead};

struct Img {
    data: Vec<Vec<u8>>,
}

impl Img {
    fn get(&self, pos: (isize, isize)) -> Option<u8> {
        if pos.0 >= 0 && pos.1 >= 0 {
            self.data.get(pos.1 as usize).and_then(|row| row.get(pos.0 as usize).map(|&v| v))
        } else {
            None
        }
    }

    fn get_mut(&mut self, pos: (isize, isize)) -> Option<&mut u8> {
        if pos.0 >= 0 && pos.1 >= 0 {
            self.data.get_mut(pos.1 as usize).and_then(|row| row.get_mut(pos.0 as usize))
        } else {
            None
        }
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn flood(&self, pos: (isize, isize)) -> usize {
        let mut visited = Img {
            data: vec![vec![0; self.width()]; self.height()]
        };
        let mut queue = LinkedList::new();
        queue.push_back(pos);
        let mut result = 0;

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            if let Some(pix) = self.get(pos) {
                if visited.get(pos) == Some(0) && pix < 9 {
                    *visited.get_mut(pos).unwrap() = 1;
                    result += 1;

                    let (x, y) = pos;
                    queue.push_back((x, y - 1));
                    queue.push_back((x - 1, y));
                    queue.push_back((x + 1, y));
                    queue.push_back((x, y + 1));
                }
            }
        }
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let data = stdin_lock.lines()
        .map(|line| { line.unwrap().bytes().map(|d| d - b'0').collect::<Vec<_>>() })
        .collect::<Vec<_>>();
    let img = Img { data };

    let mut lows = Vec::new();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let x = x as isize;
            let y = y as isize;
            let pix = img.get((x, y)).unwrap();

            let top = img.get((x, y - 1)).map(|v| v > pix).unwrap_or(true);
            let left = img.get((x - 1, y)).map(|v| v > pix).unwrap_or(true);
            let right = img.get((x + 1, y)).map(|v| v > pix).unwrap_or(true);
            let bot = img.get((x, y + 1)).map(|v| v > pix).unwrap_or(true);

            if top & left & right & bot {
                lows.push((x, y));
            }
        }
    }

    let mut areas = lows.iter().map(|&pos| img.flood(pos)).collect::<Vec<_>>();
    areas.sort_by(|a, b| b.cmp(a));
    let result = areas.iter().take(3).map(|&x| x).reduce(|x, y| x * y).unwrap();

    println!("{}", result);
}