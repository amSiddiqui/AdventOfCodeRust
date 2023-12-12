use std::fs;
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


    fn traverse_tree(line: &str, nums: &Vec<usize>) -> u64{
        if line.is_empty() {
            return if nums.is_empty() {
                1
            } else {
                0
            }
        }
        if nums.is_empty() {
            return if line.contains('#') {
                0
            } else {
                1
            };
        }
        match line.chars().nth(0) {
            Some('.') => {
                Day12::traverse_tree(&line[1..], nums)
            },
            Some('?') => {
                Day12::traverse_tree(&format!("#{}", &line[1..]), nums)
                    + Day12::traverse_tree(&line[1..], nums)
            },
            Some('#') => {
                match line.chars().nth(nums[0]) {
                    Some('#') => {
                        0
                    },
                    _ => {
                        if line.len() < nums[0] {
                            return 0;
                        }
                        if line[0..nums[0]].contains('.') {
                            return 0;
                        }
                        if line.len() == nums[0] {
                            Day12::traverse_tree("", &nums[1..].to_vec())
                        } else {
                            Day12::traverse_tree(&line[nums[0]+1..], &nums[1..].to_vec())
                        }
                    }
                }
            },
            _ => {
                panic!("Unknown character or empty line in {line}");
            }
        }
    }
}

impl Day for Day12 {
    fn part_1(&self) -> u64 {
        let res: u64 = self.lines.par_iter()
            .map(|(line, nums)| {
                Day12::traverse_tree(line, nums)
            }).sum();
        res
    }

    fn part_2(&self) -> u64 {
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
                Day12::traverse_tree(&repeated_str, &repeated_num)
            }).sum();
        res
    }
}
