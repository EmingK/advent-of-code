use std::io::{self, BufRead};

const BOARD_SIZE: usize = 5;

struct Grid {
    n: i32,
    marked: bool,
}

struct Bingo {
    board: Vec<Vec<Grid>>,
}

impl Bingo {
    fn mark(&mut self, val: i32) {
        self.board.iter_mut()
            .flat_map(|row| { row.iter_mut() })
            .filter(|grid| { grid.n == val })
            .for_each(|grid| { grid.marked = true; });
    }

    fn win(&self) -> bool {
        self.board.iter().any(|row| { row.iter().all(|grid| { grid.marked })}) ||
        (0..BOARD_SIZE).any(|idx| self.board.iter().all(|row| { row[idx].marked }))
    }

    fn sum_unmarked(&self) -> i32 {
        self.board.iter()
            .flat_map(|row| { row.iter() })
            .filter(|grid| { !grid.marked })
            .map(|grid| { grid.n })
            .reduce(|a, b| { a + b })
            .unwrap_or(0)
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut lines = stdin_lock.lines();

    let input = lines.next().unwrap().unwrap().split(',').map(|s| {s.parse::<i32>().unwrap()}).collect::<Vec<_>>();

    let board_data = lines.map(|line| {line.unwrap()}).filter(|line| {!line.is_empty()}).collect::<Vec<_>>();
    let mut boards = board_data.chunks(BOARD_SIZE).map(|group| {
        Bingo {
            board: group.iter().map(|row| {
                row.split_whitespace().map(|s| {
                    Grid {
                        n: s.parse::<i32>().unwrap(),
                        marked: false,
                    }
                }).collect()
            }).collect(),
        }
    }).collect::<Vec<_>>();

    for n in input {
        if let Some(val) = boards.iter_mut().find_map(|board| {
            board.mark(n);
            if board.win() {
                Some(n * board.sum_unmarked())
            } else {
                None
            }
        }) {
            println!("{}", val);
            break;
        }
    }
}