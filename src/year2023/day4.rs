use std::collections::{HashMap, HashSet};
use std::fs;
use crate::traits::Day;
use rayon::prelude::*;

pub struct Day4 {
    lines: Vec<u64>,
}

impl Day4 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2023/day4")
            .unwrap_or_else(|err| panic!("Error reading file;;Err {}", err));

        let lines:Vec<_> = data
            .split('\n')
            .map(|line| line.split(':')
                .skip(1)
                .collect::<String>()
            ).collect();

        let lines: Vec<_> = lines.par_iter()
            .map(|x| x.trim().split('|')
                .take(2)
                .map(String::from)
                .collect::<Vec<_>>())
            .map(|line| {
            assert_eq!(line.len(), 2, "Expected line {:?} to have 2 parts", &line);
            let nums = Day4::line_to_hashset(&line[0]);
            let winnings = Day4::line_to_hashset(&line[1]);
            winnings.intersection(&nums).count() as u64
        }).collect();

        Day4 {
            lines
        }
    }

    fn line_to_hashset(line: &String) -> HashSet<i32> {
        line.split_whitespace()
            .map(|x| x.parse::<i32>()
                .unwrap_or_else(|err| panic!("Malformed value {x};; Err {err}")))
            .collect::<HashSet<_>>()
    }
}

impl Day for Day4 {
    fn part_1(&mut self) -> u64 {
        self.lines
            .iter()
            .map(|&overlap| {
                if overlap > 0 {
                    2_u64.pow(overlap as u32 - 1)
                } else {
                    0
                }
            }).sum()
    }

    fn part_2(&mut self) -> u64 {
        let n = self.lines.len();
        let mut card_count: HashMap<_, _> = (0..n).map(|key| (key as u64, 1_u64)).collect();
        self.lines.iter().enumerate().for_each(|(i, &val)| {
            let count = *card_count.get(&(i as u64)).unwrap();
            for k in 0..val {
                let new_key = i as u64 + k + 1;
                if new_key < n as u64 {
                    *card_count.entry(new_key).or_insert(0_u64) += count;
                }
            }
        });
        card_count.values().sum()
    }
}
