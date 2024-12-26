use std::fs;

use crate::traits::Day;

const HEIGHT: u8 = 5;

pub struct Day25 {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

impl Day25 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2024/day25").expect("Cannot read data file");

        let parts = data.split("\n\n").collect::<Vec<&str>>();

        let mut keys = vec![];
        let mut locks = vec![];

        for object in parts {
            let lines = object.lines();
            let mut obj = [0; 5];
            for l in lines {
                for (i, c) in l.chars().enumerate() {
                    if c == '#' {
                        obj[i] += 1;
                    }
                }
            }

            for i in 0..5 {
                obj[i] -= 1;
            }

            // check if all values are #
            let is_key = object.lines().next().unwrap().chars().all(|c| c == '#');
            if is_key {
                keys.push(obj);
            } else {
                locks.push(obj);
            }
        }

        Day25 { keys, locks }
    }
}

impl Day for Day25 {
    fn part_1(&mut self) -> u64 {
        let mut count = 0;
        for key in &self.keys {
            for lock in &self.locks {
                if !key.iter().zip(lock.iter()).any(|(k, l)| k + l > HEIGHT) {
                    count += 1;
                }
            }
        }

        count
    }

    fn part_2(&mut self) -> u64 {
        0
    }
}
