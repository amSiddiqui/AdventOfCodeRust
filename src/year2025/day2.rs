use crate::traits::Day;
use std::fs;

pub struct Day2 {
    data: Vec<(u64, u64)>,
}

impl Day2 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day2")
        .expect("Cannot read data")
        .lines()
        .map(|line| {
            line.split(",").map(|ids| {
                let mut parts = ids.split("-");
                let first = u64::from_str_radix( parts.next().expect("first id not found"), 10).expect("Cannot parse");
                let last = u64::from_str_radix( parts.next().expect("second id not found"), 10).expect("cannot parse");
                (first, last)
            }).collect::<Vec<(u64, u64)>>()
        })
        .next().expect("No data");
        Day2 {
            data
        }
    }
}

fn is_invalid(num: &u64) -> bool {
    let num_s = num.to_string();
    if num_s.len() % 2 != 0 {
        return false;
    }
    let l = num_s.len();
    let first = &num_s[..l/2];
    let second = &num_s[l/2..];
    first == second
}

fn is_invalid_2(num: &u64) -> bool {
    let num_s = num.to_string();
    let l = num_s.len();
    for i in 1..(l/2 + 1) {
        if l % i != 0 {
            continue;
        }
        let mut no_match = false;
        let part = &num_s[..i];
        // println!("part {part}");
        let mut j = i;
        while j <= l-i {
            let sc = &num_s[j..j+i];
            // println!("sc = {sc}");
            if sc != part {
                no_match = true;
                break;
            }
            j = j+i;
        }
        if !no_match {
            return true;
        }
    }
    false
}



impl Day for Day2 {
    fn part_1(&mut self) -> u64 {
        let mut s = 0;
        for (a, b) in &self.data {
            for i in *a..(*b + 1) {
                if is_invalid(&i) {
                    
                    s += i;
                }
            }
        }
        s
    }
    fn part_2(&mut self) -> u64 {
        let mut s = 0;
        for (a, b) in &self.data {
            for i in *a..(*b + 1) {
                if is_invalid_2(&i) {
                    // println!("{i}");
                    s += i;
                }
            }
        }

        // is_invalid_2(&2121212124);
        s
    }
}