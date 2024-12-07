use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs;

use crate::traits::Day;

pub struct Day7 {
    lines: Vec<(u64, Vec<u64>)>,
}

impl Day7 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/year2024/day7")
            .expect("Cannot read data file")
            .split('\n')
            .map(|l| {
                let parts: Vec<&str> = l.split(": ").collect();
                let res: u64 = parts[0].parse().unwrap();
                let vals: Vec<u64> = (&parts)[1]
                    .split_ascii_whitespace()
                    .map(|c| c.parse().unwrap())
                    .collect();
                (res, vals)
            })
            .collect();

        Day7 { lines }
    }
}

fn check_if_any_combination(res: u64, series: &[u64]) -> bool {
    let idxs: Vec<usize> = (0..series.len() - 1).collect();
    for count_op in 0..series.len() + 1 {
        let combos = idxs.iter().combinations(count_op);
        for combo in combos {
            let mut start = match combo.contains(&&0) {
                true => series[0] * series[1],
                false => series[0] + series[1],
            };

            for i in 2..series.len() {
                if combo.contains(&&(i - 1)) {
                    start *= series[i];
                } else {
                    start += series[i];
                }
            }
            if start == res {
                return true;
            }
        }
    }
    false
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    let mut m = 1;
    let mut t = b;
    
    if t == 0 {
        return a * 10;
    }

    while t > 0 {
        t /= 10;
        m *= 10;
    }

    a * m + b
}

fn check_if_any_combination_with_concat(res: u64, series: &[u64]) -> bool {
    let ops = ['*', '+', '|'];
    let combos = (0..series.len() - 1)
        .map(|_| ops.iter().cloned())
        .multi_cartesian_product();

    for combo in combos {
        let mut start = match combo[0] {
            '*' => series[0] * series[1],
            '+' => series[0] + series[1],
            _ => concat_numbers(series[0], series[1]),
        };
        for i in 2..series.len() {
            match combo[i - 1] {
                '*' => start *= series[i],
                '+' => start += series[i],
                _ => {
                    start = concat_numbers(start, series[i]);
                }
            }
        }
        if start == res {
            return true;
        }
    }
    false
}

impl Day for Day7 {
    fn part_1(&mut self) -> u64 {
        self.lines.par_iter().filter_map(|(res, series)| {
            if check_if_any_combination(*res, series) {
                Some(res)
            } else {
                None
            }
        }).sum()
    }

    fn part_2(&mut self) -> u64 {
        self.lines.par_iter().filter_map(|(res, series)| {
            if check_if_any_combination_with_concat(*res, series) {
                Some(res)
            } else {
                None
            }
        }).sum()
    }
}
