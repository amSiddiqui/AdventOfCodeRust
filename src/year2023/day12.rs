use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use crate::traits::Day;
use rayon::prelude::*;

pub struct Day12 {
    lines: Vec<(String, Vec<usize>)>,
}

impl Day12 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day12")
            .expect("Cannot read input file")
            .split('\n')
            .filter_map(|x| x.split_once(' '))
            .map(|(line, nums_str)| {
                let line = String::from(line);
                let nums = nums_str
                    .split(',')
                    .map(|x| x.parse::<usize>().expect("Cannot parse number"))
                    .collect::<Vec<_>>();
                (line, nums)
            }).collect::<Vec<_>>();
        Day12 {
            lines
        }
    }


    fn traverse_tree(line: &str,
                     nums: &[usize],
                     cache: Arc<Mutex<HashMap<(String, Vec<usize>), u64>>>) -> u64{
        let cache_lock = cache.lock().unwrap();
        if let Some(&res) = cache_lock.get(&(line.to_string(), nums.to_vec())) {
            return res;
        }
        drop(cache_lock);
        let result =
        if line.is_empty() {
            if nums.is_empty() {
                1
            } else {
                0
            }
        }
        else if nums.is_empty() {
            if line.contains('#') {
                0
            } else {
                1
            }
        } else {
            match line.chars().nth(0) {
                Some('.') => {
                    Day12::traverse_tree(&line[1..], nums, Arc::clone(&cache))
                },
                Some('?') => {
                    Day12::traverse_tree(&format!("#{}", &line[1..]), nums, Arc::clone(&cache))
                        + Day12::traverse_tree(&line[1..], nums, Arc::clone(&cache))
                },
                Some('#') => {
                    match line.chars().nth(nums[0]) {
                        Some('#') => {
                            0
                        },
                        _ => {
                            if line.len() < nums[0] || line[0..nums[0]].contains('.') {
                                0
                            } else if line.len() == nums[0] {
                                Day12::traverse_tree("", &nums[1..], Arc::clone(&cache))
                            } else {
                                Day12::traverse_tree(&line[nums[0]+1..], &nums[1..], Arc::clone(&cache))
                            }
                        }
                    }
                },
                _ => {
                    panic!("Unknown character or empty line in {line}");
                }
            }
        };

        let mut cache_lock = cache.lock().unwrap();
        cache_lock.insert((line.to_string(), nums.to_vec()), result);
        result
    }
}

impl Day for Day12 {
    fn part_1(&self) -> u64 {
        let cache = Arc::new(Mutex::new(HashMap::new()));
        let res: u64 = self.lines.par_iter()
            .map(|(line, nums)| {
                let cache_clone = Arc::clone(&cache);
                Day12::traverse_tree(line, nums, cache_clone)
            }).sum();
        res
    }

    fn part_2(&self) -> u64 {
        let cache = Arc::new(Mutex::new(HashMap::new()));
        let res: u64 = self.lines.par_iter()
            .map(|(line, nums)| {
                let repeated_num = nums.iter()
                    .cycle()
                    .take(nums.len() * 5)
                    .cloned()
                    .collect::<Vec<_>>();
                let repeated_str = std::iter::repeat(line.as_str())
                    .take(5)
                    .collect::<Vec<_>>()
                    .join("?");
                let cache_clone = Arc::clone(&cache);
                Day12::traverse_tree(&repeated_str, &repeated_num, cache_clone)
            }).sum();
        res
    }
}
