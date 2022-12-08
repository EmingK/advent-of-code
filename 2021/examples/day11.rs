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

    fn run_step(&mut self) -> i32 {
        let mut flash_list = LinkedList::new();
        self.data.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, pix)| {
                *pix += 1;
                if *pix > 9 {
                    flash_list.push_back((x as isize, y as isize));
                }
            })
        });

        let mut result = 0;
        while let Some((x, y)) = flash_list.pop_front() {
            result += 1;
            macro_rules! inc_add {
                ($x: expr, $y: expr) => {
                    if let Some(p) = self.get_mut(($x, $y)) {
                        *p += 1;
                        if *p == 10 {
                            flash_list.push_back(($x, $y));
                        }
                    }
                }
            }

            inc_add!(x - 1, y - 1);
            inc_add!(x, y - 1);
            inc_add!(x + 1, y - 1);
            inc_add!(x - 1, y);
            inc_add!(x, y);
            inc_add!(x + 1, y);
            inc_add!(x - 1, y + 1);
            inc_add!(x, y + 1);
            inc_add!(x + 1, y + 1);
        }

        self.data.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, pix)| {
                if *pix > 9 {
                    *pix = 0;
                }
            })
        });
        result
    }

    fn is_same(&self) -> bool {
        let element = self.data[0][0];
        self.data.iter().all(|row| {
            row.iter().all(|&e| e == element)
        })
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let data = stdin_lock.lines()
        .map(|line| { line.unwrap().bytes().map(|d| d - b'0').collect::<Vec<_>>() })
        .collect::<Vec<_>>();
    let mut img = Img { data: data.clone() };

    let result = (1..=100).fold(0, |res, _| {
        res + img.run_step()
    });

    println!("{}", result);

    img = Img { data };
    let mut result = 0;
    while !img.is_same() {
        let _ = img.run_step();
        result += 1;
    }

    println!("{}", result);
}