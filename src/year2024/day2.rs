use std::fs;

use crate::traits::Day;

pub struct Day2 {
    lines: Vec<Vec<i32>>,
}

impl Day2 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/year2024/day2")
            .expect("cannot read to file")
            .split('\n')
            .map(|line| {
                line.split_whitespace()
                    .map(|i| i.parse().expect("cannot parse number"))
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        Day2 { lines }
    }

    pub fn is_safe(&self, arr: &Vec<i32>) -> bool {
        let increasing = (arr[1] - arr[0]) > 0;
        for i in 0..arr.len()-1 {
            let mut diff = arr[i+1] - arr[i];
            if increasing && diff < 0 {
                return false;
            }
            if !increasing && diff > 0 {
                return false;
            }
            if diff < 0 {
                diff *= -1;
            }
            if diff > 3 || diff < 1 {
                return false;
            }
        }
        return true;
    }
}

impl Day for Day2 {
    fn part_1(&mut self) -> u64 {
        let mut count = 0;
        for arr in &self.lines {
            if self.is_safe(arr) {
                count += 1;
            }
        }
        count
    }

    fn part_2(&mut self) -> u64 {
        let mut count = 0;
        for arr in &self.lines {
            if self.is_safe(arr) {
                count += 1;
            } else {
                for i in 0..arr.len() {
                    let mut cp = arr.clone();
                    cp.remove(i);
                    if self.is_safe(&cp) {
                        count += 1;
                        break;
                    }
                }
            }
        }
        count
    }
}
