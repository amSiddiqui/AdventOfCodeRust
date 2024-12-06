use std::{collections::HashSet, fs};

use crate::traits::Day;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum DIR {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub struct Day6 {
    lines: Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    pos_x: usize,
    pos_y: usize,
    lim_x: usize,
    lim_y: usize,
    dir: DIR,
}

impl Day6 {
    pub fn new() -> Self {
        let (mut start_x, mut start_y) = (0, 0);
        let (mut pos_x, mut pos_y) = (0, 0);
        let dir = DIR::NORTH; // know this from input
        let lines: Vec<Vec<char>> = fs::read_to_string("data/year2024/day6")
            .expect("Cannot read data file")
            .split('\n')
            .map(|l| l.chars().collect())
            .collect();

        lines.iter().enumerate().for_each(|(y, l)| {
            l.iter().enumerate().for_each(|(x, c)| {
                if ['^', '>', '<', 'v'].contains(c) {
                    start_x = x;
                    start_y = y;
                    pos_x = x;
                    pos_y = y;
                }
            });
        });

        let lim_x = (&lines)[0].len();
        let lim_y = lines.len();

        Day6 {
            lines,
            start_x,
            start_y,
            pos_x,
            pos_y,
            lim_x,
            lim_y,
            dir,
        }
    }

    pub fn is_facing_bound(&self) -> bool {
        (self.pos_x == 0 && self.dir == DIR::WEST)
            || (self.pos_x == self.lim_x - 1 && self.dir == DIR::EAST)
            || (self.pos_y == 0 && self.dir == DIR::NORTH)
            || (self.pos_y == self.lim_y - 1 && self.dir == DIR::SOUTH)
    }

    pub fn move_step(&mut self) -> bool {
        let (next_x, next_y) = match self.dir {
            DIR::NORTH => (self.pos_x, self.pos_y - 1),
            DIR::SOUTH => (self.pos_x, self.pos_y + 1),
            DIR::EAST => (self.pos_x + 1, self.pos_y),
            DIR::WEST => (self.pos_x - 1, self.pos_y),
        };

        if self.lines[next_y][next_x] == '#' {
            // turn
            self.dir = match self.dir {
                DIR::NORTH => DIR::EAST,
                DIR::EAST => DIR::SOUTH,
                DIR::SOUTH => DIR::WEST,
                DIR::WEST => DIR::NORTH,
            };
            false
        } else {
            self.pos_x = next_x;
            self.pos_y = next_y;
            true
        }
    }

    pub fn reset(&mut self) {
        self.pos_x = self.start_x;
        self.pos_y = self.start_y;
        self.dir = DIR::NORTH;
    }
}

impl Day for Day6 {
    fn part_1(&mut self) -> u64 {
        self.reset();
        let mut count = 1;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((self.pos_x, self.pos_y));
        while !self.is_facing_bound() {
            self.move_step();
            if !visited.contains(&(self.pos_x, self.pos_y)) {
                count += 1;
                visited.insert((self.pos_x, self.pos_y));
            }
        }
        count
    }

    fn part_2(&mut self) -> u64 {
        let mut count = 0;

        for y in 0..self.lim_y {
            for x in 0..self.lim_x {
                self.reset();
                if x == self.start_x && y == self.start_y {
                    continue;
                }
                if self.lines[y][x] == '#' {
                    continue;
                }
                
                // place block
                self.lines[y][x] = '#';

                let mut visited: HashSet<(usize, usize, DIR)> = HashSet::new();
                visited.insert((self.pos_x, self.pos_y, self.dir));
                while !self.is_facing_bound() {
                    self.move_step();
                    if visited.contains(&(self.pos_x, self.pos_y, self.dir)) {
                        count += 1;
                        break;
                    }
                    visited.insert((self.pos_x, self.pos_y, self.dir));
                }

                self.lines[y][x] = '.';
            }
        }

        count
    }
}
