use crate::traits::Day;
use regex::Regex;
use std::{fs, usize};

pub struct Day3 {
    line: String,
}

fn find_closest_smaller(vec: &[usize], target: usize) -> Option<usize> {
    vec.iter().filter(|&&x| x < target).max().copied()
}

impl Day3 {
    pub fn new() -> Self {
        let line = fs::read_to_string("data/year2024/day3").expect("cannot read data");
        Day3 { line }
    }
}

impl Day for Day3 {
    fn part_1(&mut self) -> u64 {
        let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
        let re = Regex::new(&pattern).unwrap();
        let mut sum = 0;
        for cap in re.captures_iter(&self.line) {
            let a: u64 = cap.get(1).unwrap().as_str().parse().expect("cannot parse");
            let b: u64 = cap.get(2).unwrap().as_str().parse().expect("cannot parse");
            sum += a * b;
        }
        sum
    }

    fn part_2(&mut self) -> u64 {
        let do_pat = r"do\(\)";
        let dont_pat = r"don't\(\)";
        let do_re = Regex::new(&do_pat).unwrap();
        let dont_re = Regex::new(&dont_pat).unwrap();

        let mut do_idx = vec![];
        let mut dont_idx = vec![];

        for mat in do_re.find_iter(&self.line) {
            do_idx.push(mat.start());
        }

        for mat in dont_re.find_iter(&self.line) {
            dont_idx.push(mat.start());
        }
        let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
        let re = Regex::new(&pattern).unwrap();

        let mut sum = 0;
        for cap in re.captures_iter(&self.line) {
            let cap_a = cap.get(1).unwrap();
            let pos = cap_a.start();
            let a: u64 = cap_a.as_str().parse().expect("cannot parse");
            let b: u64 = cap.get(2).unwrap().as_str().parse().expect("cannot parse");
            if pos < do_idx[0] && pos < dont_idx[0] {
                sum += a * b;
                continue;
            }

            let d_i = find_closest_smaller(&do_idx, pos).unwrap_or(0);
            let dnt_i = find_closest_smaller(&dont_idx, pos).unwrap_or(0);
        
            if d_i > dnt_i {
                sum += a*b;
            }

        }
        sum
    }
}
