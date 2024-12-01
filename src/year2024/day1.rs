use crate::traits::Day;
use std::{collections::HashMap, fs};

pub struct Day1 {
    list1: Vec<u64>,
    list2: Vec<u64>,
}

impl Day1 {
    pub fn new() -> Self {
        let mut list1 = vec![];
        let mut list2 = vec![];
        fs::read_to_string("data/year2024/day1")
            .expect("cannot read data")
            .split('\n')
            .for_each(|line| {
                let mut parts = line.split_ascii_whitespace();
                let n1 = u64::from_str_radix(parts.next().expect("cannot extract line"), 10)
                    .expect("cannot parse line");
                let n2 = u64::from_str_radix(parts.next().expect("cannot extract line"), 10)
                    .expect("cannot parse line");
                list1.push(n1);
                list2.push(n2);
            });

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
