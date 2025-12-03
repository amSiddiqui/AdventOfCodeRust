use crate::traits::Day;
use std::fs;

pub struct Day3 {
    data: Vec<Vec<u32>>,
}

impl Day3 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day3")
            .expect("Cannot read data")
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        Day3 { data }
    }
}

fn largest_digit(line: &Vec<u32>, start: usize, end: usize) -> usize {
    let mut last_digit = 0;
    let mut large_idx = 0;
    for i in start..end {
        let digit = line[i];
        if digit > last_digit {
            last_digit = digit;
            large_idx = i;
        }
    }
    large_idx
}

impl Day for Day3 {
    fn part_1(&mut self) -> u64 {
        let mut res = 0;
        for line in &self.data {
            let l = line.len();
            let n = largest_digit(line, 0, l - 1);
            let m = largest_digit(line, n + 1, l);
            let num = 10 * line[n] + line[m];
            res += num;
        }

        res as u64
    }
    fn part_2(&mut self) -> u64 {
        let mut res = 0;
        for line in &self.data {
            let mut digits: Vec<u32> = vec![];
            let l = line.len();
            let mut prev_n = 0;
            for i in 0..12 {
                let n = largest_digit(line, prev_n, l - (12 - i - 1));
                prev_n = n + 1;
                digits.push(line[n]);
            }
            let mut num: u64 = 0;
            for d in digits {
                num = num * 10 + (d as u64);
            }
            res += num;
            // dbg!(num);
        }

        res as u64
    }
}
