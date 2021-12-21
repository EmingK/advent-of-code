use std::io::{self, BufRead};

struct Led {
    map: [u8; 128],
}

impl Led {
    fn parse(repr: &str) -> Self {
        let mut map = [0u8; 128];
        let mut nums = [0u8; 10];

        let mut five_strokes = [0u8; 3];
        let mut six_strokes = [0u8; 3];

        let mut five = five_strokes.iter_mut();
        let mut six = six_strokes.iter_mut();

        repr.trim().split(' ').for_each(|s| {
            let n = Self::inner_repr(s);
            match s.len() {
                2 => nums[1] = n,
                3 => nums[7] = n,
                4 => nums[4] = n,
                7 => nums[8] = n,
                5 => *five.next().unwrap() = n, // 2, 3, 5
                6 => *six.next().unwrap() = n, // 6, 9, 0
                _ => (),
            }
        });

        // find 3
        nums[3] = *five_strokes.iter().find(|&&n| n & nums[1] == nums[1]).unwrap();
        // find 5
        let five_pat = nums[1] ^ nums[4];
        nums[5] = *five_strokes.iter().find(|&&n| n & five_pat == five_pat).unwrap();
        // find 2
        nums[2] = *five_strokes.iter().find(|&&n| n != nums[3] && n != nums[5]).unwrap();

        // find 6
        nums[6] = *six_strokes.iter().find(|&&n| n & nums[1] != nums[1]).unwrap();
        // find 9
        nums[9] = *six_strokes.iter().find(|&&n| n & nums[3] == nums[3]).unwrap();
        // find 0
        nums[0] = *six_strokes.iter().find(|&&n| n != nums[6] && n != nums[9]).unwrap();

        nums.iter().enumerate().for_each(|(n, idx)| {
            //println!("{} {:08b}", n, idx);
            map[*idx as usize] = n as u8;
        });

        Self {
            map
        }
    }

    fn query(&self, s: &str) -> u8 {
        let idx = Self::inner_repr(s) as usize;
        self.map[idx]
    }

    fn query_multi(&self, s: &str) -> i32 {
        s.trim().split(' ').fold(0, |res, digit| {
            res * 10 + self.query(digit) as i32
        })
    }

    fn inner_repr(val: &str) -> u8 {
        val.bytes().fold(0, |res, byte| {
            res | match byte {
                b'a' => 0b0000001,
                b'b' => 0b0000010,
                b'c' => 0b0000100,
                b'd' => 0b0001000,
                b'e' => 0b0010000,
                b'f' => 0b0100000,
                b'g' => 0b1000000,
                _ => 0,
            }
        })
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let lines = stdin_lock.lines().map(|line| { line.unwrap() });

    let result = lines.fold(0, |res, line| {
        let mut parts = line.split('|');
        let led = Led::parse(parts.next().unwrap());
        let query = parts.next().unwrap();
        let decoded = led.query_multi(query);
        res + decoded
    });
    println!("{}", result);
}