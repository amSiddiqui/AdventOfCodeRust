use std::{collections::HashSet, fs};

use crate::traits::Day;

pub struct Day14 {
    data: Vec<((i32, i32), (i32, i32))>,
    lim: (i32, i32),
}

fn parse_str(part: &str) -> (i32, i32) {
    let mut parts = part[2..].split(',');
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

impl Day14 {
    pub fn new() -> Self {
        Day14 {
            data: Vec::new(),
            lim: (100, 102),
        }
    }

    pub fn load(&mut self) {
        let data = fs::read_to_string("data/year2024/day14")
            .expect("cannot read data file")
            .lines()
            .map(|l| {
                let mut parts = l.split_ascii_whitespace();
                let pos = parts.next().unwrap();
                let vel = parts.next().unwrap();
                (parse_str(pos), parse_str(vel))
            })
            .collect();
        self.data = data;
    }

    pub fn step(&mut self) {
        for (pos, vel) in self.data.iter_mut() {
            pos.0 += vel.0;
            pos.1 += vel.1;

            if pos.0 < 0 {
                pos.0 += self.lim.0 + 1;
            }

            if pos.0 > self.lim.0 {
                pos.0 = pos.0 - self.lim.0 - 1;
            }

            if pos.1 < 0 {
                pos.1 += self.lim.1 + 1;
            }
            if pos.1 > self.lim.1 {
                pos.1 = pos.1 - self.lim.1 - 1;
            }
        }
    }

    pub fn is_tree(&self) -> bool {
        let set: HashSet<(i32, i32)> = self.data.iter().map(|(pos, _)| (pos.0, pos.1)).collect();
        for (start, _) in self.data.iter() {
            let mut left = start.clone();
            let mut right = start.clone();
            let mut found = false;
            for _ in 0..5 {
                if set.contains(&(left.0 - 1, left.1 + 1))
                    && set.contains(&(right.0 + 1, right.1 + 1))
                {
                    left = (left.0 - 1, left.1 + 1);
                    right = (right.0 + 1, right.1 + 1);
                } else {
                    found = false;
                    break;
                }
                found = true;
            }
            if found {
                return true;
            }
        }

        false
    }
}

impl Day for Day14 {
    fn part_1(&mut self) -> u64 {
        self.load();
        for _ in 0..100 {
            self.step();
        }

        // quadrant 1
        let quads = [
            ((0, 0), (self.lim.0 / 2 - 1, self.lim.1 / 2 - 1)),
            ((self.lim.0 / 2 + 1, 0), (self.lim.0, self.lim.1 / 2 - 1)),
            ((0, self.lim.1 / 2 + 1), (self.lim.0 / 2 - 1, self.lim.1)),
            (
                (self.lim.0 / 2 + 1, self.lim.1 / 2 + 1),
                (self.lim.0, self.lim.1),
            ),
        ];

        let mut total = 1;
        for (low, high) in quads {
            let res = self
                .data
                .iter()
                .filter(|((x, y), _)| *x >= low.0 && *y >= low.1 && *x <= high.0 && *y <= high.1)
                .count();

            total *= res;
        }

        total as u64
    }

    fn part_2(&mut self) -> u64 {
        self.load();
        let mut count = 0;
        while !self.is_tree() {
            self.step();
            count += 1;
        }
        // print_out
        let mut grid = vec![vec!['.'; 101]; 103];
        for (pos, _) in self.data.iter() {
            grid[pos.1 as usize][pos.0 as usize] = '*';
        }
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                print!("{}", grid[y][x]);
            }
            println!();
        }

        return count;
    }
}
