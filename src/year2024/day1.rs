use crate::traits::Day;
use std::{collections::HashMap, fs};

pub struct Day1 {
    list1: Vec<u64>,
    list2: Vec<u64>,
}

impl Day1 {
    pub fn new() -> Self {
        let (mut list1, mut list2): (Vec<u64>, Vec<u64>) = fs::read_to_string("data/year2024/day1")
            .expect("Cannot read data")
            .lines()
            .filter_map(|line| {
                let mut parts = line.split_ascii_whitespace();
                let n1: u64 = parts.next()?.parse().ok()?;
                let n2: u64 = parts.next()?.parse().ok()?;
                Some((n1, n2))
            })
            .unzip();

        list1.sort();
        list2.sort();
        Day1 { list1, list2 }
    }
}

impl Day for Day1 {
    fn part_1(&mut self) -> u64 {
        self.list1
            .iter()
            .zip(self.list2.iter())
            .map(|(n1, n2)| n1.abs_diff(*n2))
            .sum()
    }

    fn part_2(&mut self) -> u64 {
        let mut freq_count: HashMap<u64, u64> = HashMap::new();
        for n in &self.list2 {
            *freq_count.entry(*n).or_insert(0) += 1;
        }

        self.list1
            .iter()
            .map(|n| n * freq_count.get(n).unwrap_or(&0))
            .sum()
    }
}
