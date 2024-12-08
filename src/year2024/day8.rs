use crate::traits::Day;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub struct Day8 {
    lines: Vec<Vec<char>>,
    freqs: HashMap<char, Vec<(i32, i32)>>,
}

impl Day8 {
    pub fn new() -> Self {
        let lines: Vec<Vec<char>> = fs::read_to_string("data/year2024/day8")
            .expect("cannot read data file")
            .split('\n')
            .map(|l| l.chars().collect())
            .collect();
        let mut freqs = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if *ch == '.' {
                    continue;
                }
                (*freqs.entry(*ch).or_insert(vec![])).push((x as i32, y as i32));
            }
        }

        Day8 { lines, freqs }
    }
}

fn point_in_bound(point: &(i32, i32), bound: &(i32, i32)) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < bound.0 && point.1 < bound.1
}

impl Day for Day8 {
    fn part_1(&mut self) -> u64 {
        let mut antinodes = HashSet::new();
        let x_lim = self.lines[0].len() as i32;
        let y_lim = self.lines.len() as i32;
        let bound = (x_lim, y_lim);
        for arr in self.freqs.values() {
            for pair in arr.iter().combinations(2) {
                let a = pair[0];
                let b = pair[1];

                let y_diff = b.1 - a.1;
                let x_diff = b.0 - a.0;

                let p1 = (a.0 - x_diff, a.1 - y_diff);
                let p2 = (b.0 + x_diff, b.1 + y_diff);
                if point_in_bound(&p1, &bound) {
                    antinodes.insert(p1);
                }

                if point_in_bound(&p2, &bound) {
                    antinodes.insert(p2);
                }
            }
        }
        antinodes.len() as u64
    }

    fn part_2(&mut self) -> u64 {
        let mut antinodes = HashSet::new();
        let x_lim = self.lines[0].len() as i32;
        let y_lim = self.lines.len() as i32;
        let bound = (x_lim, y_lim);
        for arr in self.freqs.values() {
            antinodes.extend(arr.iter().cloned());
            for pair in arr.iter().combinations(2) {
                let a = pair[0];
                let b = pair[1];

                let y_diff = b.1 - a.1;
                let x_diff = b.0 - a.0;

                let mut p1 = (a.0 - x_diff, a.1 - y_diff);
                let mut p2 = (b.0 + x_diff, b.1 + y_diff);
                while point_in_bound(&p1, &bound) {
                    antinodes.insert(p1);
                    p1 = (p1.0 - x_diff, p1.1 - y_diff);
                }

                while point_in_bound(&p2, &bound) {
                    antinodes.insert(p2);
                    p2 = (p2.0 + x_diff, p2.1 + y_diff);
                }
            }
        }
        antinodes.len() as u64
    }
}
