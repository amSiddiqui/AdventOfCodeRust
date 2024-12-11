use std::{collections::HashMap, fs};

use crate::traits::Day;

pub struct Day11 {
    numbers: Vec<u64>,
}

impl Day11 {
    pub fn new() -> Self {
        let numbers: Vec<u64> = fs::read_to_string("data/year2024/day11")
            .expect("cannot read data file")
            .split_ascii_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();

        Day11 { numbers }
    }
}

pub fn split_digit_if_even(num: u64) -> Option<(u64, u64)> {
    let mut n = 0;
    let mut t = num;
    while t > 0 {
        n += 1;
        t /= 10;
    }

    if n % 2 != 0 {
        return None;
    }

    let po = 10u64.pow(n / 2);
    let a = num % po;
    let b = num / po;
    Some((b, a))
}

pub fn apply_rule(numbers: Vec<u64>) -> Vec<u64> {
    let mut new_nums = Vec::with_capacity(numbers.len());
    for num in numbers {
        if num == 0 {
            new_nums.push(1);
            continue;
        }
        if let Some((a, b)) = split_digit_if_even(num) {
            new_nums.push(a);
            new_nums.push(b);
        } else {
            if num >= u64::MAX / 2024 {
                panic!("{num} overflows");
            }
            new_nums.push(num * 2024);
        }
    }
    new_nums
}

fn count_res(start: u64, step: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if step == 0 {
        return 1;
    }
    if let Some(res) = cache.get(&(start, step)) {
        return *res;
    }

    if start == 0 {
        let res = count_res(1, step-1, cache);
        cache.insert((0, step), res);
        return res;
    }

    if let Some((a, b)) = split_digit_if_even(start) {
        let res1 = count_res(a, step - 1, cache);
        let res2 = count_res(b, step - 1, cache);
        cache.insert((start, step), res1 + res2);
        return res1 + res2;
    }

    let res = count_res(start * 2024, step - 1, cache);
    cache.insert((start, step), res);

    res
}



impl Day for Day11 {
    fn part_1(&mut self) -> u64 {
        let mut start = self.numbers.clone();
        let mut freq_map = HashMap::new();
        for s in start.iter() {
            *freq_map.entry(*s).or_insert(0) += 1;
        }
        for _ in 0..25 {
            start = apply_rule(start);
            for s in start.iter() {
                *freq_map.entry(*s).or_insert(0) += 1;
            }
        }

        start.len() as u64
    }

    fn part_2(&mut self) -> u64 {
        let mut cache: HashMap<(u64, u8), u64> = HashMap::new();
        self.numbers.iter().map(|n| {
            count_res(*n, 75, &mut cache)
        }).sum()
    }
}
