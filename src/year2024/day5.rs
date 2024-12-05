use std::{collections::HashMap, fs};

use rayon::prelude::*;
use crate::traits::Day;

pub struct Day5 {
    rules: HashMap<u64, Vec<u64>>,
    pages: Vec<Vec<u64>>,
}

impl Day5 {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        let parts: Vec<String> = fs::read_to_string("data/year2024/day5")
            .expect("Cannot read data file")
            .split("\n\n")
            .map(String::from)
            .collect();

        parts[0].split('\n').for_each(|ln| {
            let nums: Vec<u64> = ln
                .split('|')
                .map(|n| n.parse().expect("Not digit"))
                .collect();
            (*rules.entry(nums[0]).or_insert(vec![])).push(nums[1]);
        });

        let pages = parts[1]
            .split('\n')
            .map(|ln| {
                ln.split(',')
                    .map(|n| n.parse().expect("Not digit"))
                    .collect()
            })
            .collect();

        Day5 { rules, pages }
    }

    pub fn is_valid(&self, i: usize) -> bool {
        let mut idx_map: HashMap<u64, usize> = HashMap::new();
        for (idx, val) in self.pages[i].iter().enumerate() {
            idx_map.insert(*val, idx);
        }

        for (idx, val) in self.pages[i].iter().enumerate() {
            if let Some(pairs) = self.rules.get(val) {
                for pair in pairs {
                    if let Some(pair_idx) = idx_map.get(pair) {
                        if *pair_idx < idx {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn make_valid_page(rules: &HashMap<u64, Vec<u64>>, page: &mut Vec<u64>) -> u64 {
        let mut idx_map: HashMap<u64, usize> = HashMap::new();
        for (idx, val) in page.iter().enumerate() {
            idx_map.insert(*val, idx);
        }
        let mut was_invalid = false;
        loop {
            let mut swap_pairs = vec![];
            for (idx, val) in page.iter().enumerate() {
                if let Some(pairs) = rules.get(val) {
                    for pair in pairs {
                        if let Some(pair_idx) = idx_map.get(pair) {
                            if *pair_idx < idx {
                                swap_pairs.push((*pair_idx, idx));
                                was_invalid = true;
                            }
                        }
                    }
                }
                if !swap_pairs.is_empty() {
                    break;
                }
            }
            if swap_pairs.is_empty() {
                break;
            } else {
                for (i, j) in swap_pairs {
                    page.swap(i, j);
                    idx_map.insert(page[i], i);
                    idx_map.insert(page[j], j);
                }
            }
        }
        if was_invalid {
            page[page.len() / 2]
        } else {
            0
        }
    }
}

impl Day for Day5 {
    fn part_1(&mut self) -> u64 {
        let mut sum = 0;

        for (i, pg) in self.pages.iter().enumerate() {
            if self.is_valid(i) {
                sum += pg[pg.len() / 2];
            }
        }
        sum
    }

    fn part_2(&mut self) -> u64 {
        let rules = &self.rules;
        self.pages
            .par_iter_mut()
            .map(|page| Day5::make_valid_page(rules, page))
            .sum()
    }
}
