use std::fs;
use std::collections::HashSet;
use crate::traits::Day;

pub struct Day11 {
    points: Vec<(usize, usize)>,
    empty_x: Vec<usize>,
    empty_y: Vec<usize>
}

impl Day11 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/year2023/day11")
            .expect("Cannot read input file")
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>();

        let mut points = Vec::new();
        let mut used_x = HashSet::new();
        let mut used_y = HashSet::new();
        for (x, line) in lines.iter().enumerate() {
            for (y, tile) in line.chars().enumerate() {
                if tile == '#' {
                    points.push((x, y));
                    used_x.insert(x);
                    used_y.insert(y);
                }
            }
        }

        let all_x = (0..lines.len()).collect::<HashSet<usize>>();
        let all_y = (0..lines[0].len()).collect::<HashSet<usize>>();
        let mut empty_x = all_x.difference(&used_x)
            .cloned()
            .collect::<Vec<_>>();
        let mut empty_y = all_y.difference(&used_y)
            .cloned()
            .collect::<Vec<_>>();
        empty_x.sort();
        empty_y.sort();
        Day11 {
            points,
            empty_x,
            empty_y
        }
    }

    fn get_distance_sum(arr: &Vec<(u64, u64)>) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..arr.len() - 1 {
            for j in (i+1)..arr.len() {
                sum += arr[i].0.abs_diff(arr[j].0) + arr[i].1.abs_diff(arr[j].1);
            }
        }
        sum
    }
}

impl Day for Day11 {
    fn part_1(&mut self) -> u64 {
        let mut new_positions: Vec<(u64, u64)> = Vec::new();
        for pos in &self.points {
            let x = pos.0 as u64 + self.empty_x.binary_search(&pos.0).unwrap_or_else(|inc| inc) as u64;
            let y = pos.1 as u64 + self.empty_y.binary_search(&pos.1).unwrap_or_else(|inc| inc) as u64;
            new_positions.push((x, y));
        }

        Day11::get_distance_sum(&new_positions)
    }

    fn part_2(&mut self) -> u64 {
        let mut new_positions: Vec<(u64, u64)> = Vec::new();
        for pos in &self.points {
            let x = pos.0 as u64 + (self.empty_x.binary_search(&pos.0).unwrap_or_else(|inc| inc) as u64) * (1_000_000_u64 - 1);
            let y = pos.1 as u64 + (self.empty_y.binary_search(&pos.1).unwrap_or_else(|inc| inc) as u64) * (1_000_000_u64 - 1);
            new_positions.push((x, y));
        }
        Day11::get_distance_sum(&new_positions)
    }
}
