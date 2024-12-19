use dashmap::DashMap;
use rayon::prelude::*;
use std::sync::Arc;
use std::{collections::HashSet, fs};

use crate::traits::Day;

pub struct Day19 {
    towels: HashSet<String>,
    patterns: Vec<String>,
    max: usize,
}

impl Day19 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2024/day19").expect("Cannot read data file");
        let mut parts = data.lines();
        let towels = parts
            .next()
            .unwrap()
            .split(", ")
            .map(String::from)
            .collect::<Vec<String>>();
        parts.next();
        let patterns = parts.map(String::from).collect::<Vec<String>>();
        let towels: HashSet<String> = towels.iter().cloned().collect();
        let max = towels
            .iter()
            .map(|towel| towel.len())
            .max()
            .expect("No towels found");

        Day19 {
            towels,
            patterns,
            max,
        }
    }
}


// Note: This is something new that I got to learn while adding cache to multithreaded approach
// DashMap which is thread-safe HashMap implementation
fn dfs(towels: &HashSet<String>, pattern: &str, max: usize, cache: &DashMap<String, bool>) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if let Some(result) = cache.get(pattern).map(|v| *v.value()) {
        return result;
    }
    for end in 1..=max.min(pattern.len()) {
        if towels.contains(&pattern[..end]) && dfs(towels, &pattern[end..], max, cache) {
            cache.insert(pattern.to_string(), true);
            return true;
        }
    }
    cache.insert(pattern.to_string(), false);
    false
}

fn dfs2(towels: &HashSet<String>, pattern: &str, max: usize, cache: &DashMap<String, u64>) -> u64 {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(pattern).map(|v| *v.value()) {
        return count;
    }
    let count = (1..=max.min(pattern.len()))
        .filter(|&end| towels.contains(&pattern[..end]))
        .map(|end| dfs2(towels, &pattern[end..], max, cache))
        .sum();
    cache.insert(pattern.to_string(), count);
    count
}


impl Day for Day19 {
    fn part_1(&mut self) -> u64 {
        let cache = Arc::new(DashMap::new());
        self.patterns
            .par_iter()
            .filter(|pattern| {
                let cache = Arc::clone(&cache);
                dfs(&self.towels, *pattern, self.max, &cache)
            })
            .count() as u64
    }

    fn part_2(&mut self) -> u64 {
        let cache = Arc::new(DashMap::new());
        self.patterns
            .par_iter()
            .map(|pattern| {
                let cache = Arc::clone(&cache);
                dfs2(&self.towels, pattern, self.max, &cache)
            })
            .sum()
    }
}
