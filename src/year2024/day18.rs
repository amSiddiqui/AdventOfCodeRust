use std::{
    collections::{HashMap, HashSet},
    fs,
};
use rayon::prelude::*;

use crate::traits::Day;

pub struct Day18 {
    bytes: Vec<(usize, usize)>,
    x_lim: usize,
}

impl Day18 {
    pub fn new() -> Self {
        let bytes = fs::read_to_string("data/year2024/day18")
            .expect("Cannot read data file")
            .lines()
            .map(|x| {
                let mut parts = x.split(',');
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        Day18 { bytes, x_lim: 70 }
    }
}

pub fn dfs(
    bytes: &HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    count: u64,
    lim: usize,
    visited: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if x == lim && y == lim {
        return count;
    }

    if let Some(steps) = visited.get(&(x, y)) {
        if *steps <= count {
            return u64::MAX;
        }
    }

    visited.insert((x, y), count);

    if bytes.contains(&(x, y)) {
        return u64::MAX;
    }

    let mut all_dir = vec![];

    if x > 0 {
        all_dir.push(dfs(bytes, x - 1, y, count + 1, lim, visited));
    }

    if x < lim {
        all_dir.push(dfs(bytes, x + 1, y, count + 1, lim, visited));
    }

    if y > 0 {
        all_dir.push(dfs(bytes, x, y - 1, count + 1, lim, visited));
    }

    if y < lim {
        all_dir.push(dfs(bytes, x, y + 1, count + 1, lim, visited));
    }

    *all_dir.iter().min().unwrap_or(&u64::MAX)
}

impl Day for Day18 {
    fn part_1(&mut self) -> u64 {
        let sim = 1024;
        let bytes: HashSet<(usize, usize)> = self.bytes.iter().take(sim).cloned().collect();
        let mut visited = HashMap::new();
        dfs(&bytes, 0, 0, 0, self.x_lim, &mut visited)
    }

    fn part_2(&mut self) -> u64 {
        let result = (0..self.bytes.len())
            .into_par_iter()
            .find_any(|&sim| {
                let bytes: HashSet<(usize, usize)> = self.bytes.iter().take(sim).cloned().collect();
                let mut visited = HashMap::new();
                let dfs_result = dfs(&bytes, 0, 0, 0, self.x_lim, &mut visited);
                if dfs_result == u64::MAX {
                    let res = self.bytes[sim-1];
                    println!("{},{}", res.0, res.1);
                    true
                } else {
                    false
                }
            });
        if let Some(_) = result {
            0
        } else {
            unimplemented!()
        }
    }
}
