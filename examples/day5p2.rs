use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter::{empty, repeat};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    ParseError,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::ParseError
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point { x: i32, y: i32 }

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or(Error::ParseError)?.parse()?;
        let y = parts.next().ok_or(Error::ParseError)?.parse()?;
        Ok(Point { x, y })
    }
}

struct PointPair { from: Point, to: Point }

impl PointPair {
    fn line_points(&self) -> Box<dyn Iterator<Item = Point>> {
        let x1 = i32::min(self.from.x, self.to.x);
        let x2 = i32::max(self.from.x, self.to.x);
        let y1 = i32::min(self.from.y, self.to.y);
        let y2 = i32::max(self.from.y, self.to.y);
        if self.from.x == self.to.x {
            Box::new(
                repeat(self.from.x).zip(y1..=y2).map(|(x, y)| { Point { x, y } })
            )
        } else if self.from.y == self.to.y {
            Box::new(
                (x1..=x2).zip(repeat(self.from.y)).map(|(x, y)| { Point { x, y } })
            )
        } else if (x2 - x1) == (y2 - y1) {
            let x: Box<dyn Iterator<Item = i32>> = if self.from.x > self.to.x {
                Box::new((x1..=x2).rev())
            } else {
                Box::new(x1..=x2)
            };
            let y: Box<dyn Iterator<Item = i32>> = if self.from.y > self.to.y {
                Box::new((y1..=y2).rev())
            } else {
                Box::new(y1..=y2)
            };
            Box::new(
                x.zip(y).map(|(x, y)| { Point { x, y } })
            )
        } else {
            Box::new(empty())
        }
    }
}

impl FromStr for PointPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("->");
        let from = parts.next().ok_or(Error::ParseError)?.trim().parse()?;
        let to = parts.next().ok_or(Error::ParseError)?.trim().parse()?;
        Ok(PointPair { from, to })
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let lines = stdin_lock.lines().map(|line| { line.unwrap() });

    let mut points: HashMap<Point, i32> = HashMap::new();

    lines.for_each(|line| {
        let pair: PointPair = line.parse().unwrap();
        pair.line_points().for_each(|pt| {
            let mut count = *(points.get(&pt).unwrap_or(&0));
            count += 1;
            points.insert(pt, count);
        });
    });

    let result = points.values().filter(|overlap| { **overlap >= 2 }).count();
    println!("{}", result);
}