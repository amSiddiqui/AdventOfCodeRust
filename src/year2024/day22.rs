use std::{fs, u8};

use crate::traits::Day;
use ahash::{HashMap, HashSet, RandomState};
use rayon::prelude::*;

pub struct Day22 {
    numbers: Vec<u64>,
}

impl Day22 {
    pub fn new() -> Self {
        let numbers = fs::read_to_string("data/year2024/day22")
            .expect("Cannot read data file")
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        Day22 { numbers }
    }
}

fn process(input: u64) -> u64 {
    let a = ((input * 64) ^ input) % 16777216;
    let b = ((a / 32) ^ a) % 16777216;
    let c = ((b * 2048) ^ b) % 16777216;
    c
}

impl Day for Day22 {
    fn part_1(&mut self) -> u64 {
        self.numbers
            .par_iter()
            .map(|num| {
                let mut n = *num;
                for _ in 0..2000 {
                    n = process(n);
                }
                n
            })
            .sum()
    }

    fn part_2(&mut self) -> u64 {
        let digits: Vec<Vec<u8>> = self
            .numbers
            .iter()
            .map(|&num| {
                let mut v = Vec::with_capacity(2001);
                let mut n = num;
                v.push((n % 10) as u8);
                for _ in 0..2000 {
                    n = process(n);
                    v.push((n % 10) as u8);
                }
                v
            })
            .collect();

        let diff_patterns: Vec<Vec<i8>> = digits
            .iter()
            .map(|d| {
                d.windows(2)
                    .map(|pair| pair[1] as i8 - pair[0] as i8)
                    .collect()
            })
            .collect();

        let mut pattern_maps = Vec::with_capacity(diff_patterns.len());
        for (j, diffs_j) in diff_patterns.iter().enumerate() {
            let s = RandomState::new();
            let mut map: HashMap<[i8; 4], u8> = HashMap::with_capacity_and_hasher(diffs_j.len().saturating_sub(3), s);
            let digits_j = &digits[j];
            for l in 0..diffs_j.len().saturating_sub(3) {
                let pattern = [diffs_j[l], diffs_j[l + 1], diffs_j[l + 2], diffs_j[l + 3]];
                let candidate = digits_j[l + 4];
                map.entry(pattern).or_insert(candidate);
            }
            pattern_maps.push(map);
        }

        digits
            .iter()
            .enumerate()
            .par_bridge()
            .map(|(i, digits_i)| {
                let diffs_i = &diff_patterns[i];
                let mut best_for_i = 0u64;
                let mut visited_patterns = HashSet::default();
                for k in 0..diffs_i.len().saturating_sub(3) {
                    let base_val = digits_i[k + 4] as u64;
                    let pattern = [diffs_i[k], diffs_i[k + 1], diffs_i[k + 2], diffs_i[k + 3]];
                    if visited_patterns.contains(&pattern) {
                        continue;
                    }
                    visited_patterns.insert(pattern);
                    let mut sum = base_val;

                    for j in 0..digits.len() {
                        if j == i {
                            continue;
                        }
                        if let Some(&matching_digit) = pattern_maps[j].get(&pattern) {
                            sum += matching_digit as u64;
                        }
                    }

                    best_for_i = best_for_i.max(sum);
                }
                best_for_i
            })
            .max()
            .expect("No max found")
    }
}
