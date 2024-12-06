use rayon::prelude::*;
use std::sync::Arc;
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
    lines: Arc<Vec<Vec<char>>>,
    start_x: usize,
    start_y: usize,
    lim_x: usize,
    lim_y: usize,
    dir: DIR,
}

impl Day6 {
    pub fn new() -> Self {
        let (mut start_x, mut start_y) = (0, 0);
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
                }
            });
        });

        let lim_x = (&lines)[0].len();
        let lim_y = lines.len();

        Day6 {
            lines: Arc::new(lines),
            start_x,
            start_y,
            lim_x,
            lim_y,
            dir,
        }
    }

    fn is_facing_bound(&self, x: usize, y: usize, dir: DIR) -> bool {
        (x == 0 && dir == DIR::WEST)
            || (x == self.lim_x - 1 && dir == DIR::EAST)
            || (y == 0 && dir == DIR::NORTH)
            || (y == self.lim_y - 1 && dir == DIR::SOUTH)
    }

    fn move_step(
        &self,
        lines: &Vec<Vec<char>>,
        pos_x: &mut usize,
        pos_y: &mut usize,
        dir: &mut DIR
    ) -> bool {
        let (next_x, next_y) = match dir {
            DIR::NORTH => (*pos_x, *pos_y - 1),
            DIR::SOUTH => (*pos_x, *pos_y + 1),
            DIR::EAST => (*pos_x + 1, *pos_y),
            DIR::WEST => (*pos_x - 1, *pos_y),
        };

        if lines[next_y][next_x] == '#' {
            *dir = match dir {
                DIR::NORTH => DIR::EAST,
                DIR::EAST => DIR::SOUTH,
                DIR::SOUTH => DIR::WEST,
                DIR::WEST => DIR::NORTH,
            };
            false
        } else {
            *pos_x = next_x;
            *pos_y = next_y;
            true
        }
    }

    fn run_simulation(
        &self,
        mut lines: Vec<Vec<char>>,
        block_x: usize,
        block_y: usize
    ) -> bool {

        // place block
        lines[block_y][block_x] = '#';

        let mut pos_x = self.start_x;
        let mut pos_y = self.start_y;
        let mut dir = self.dir;

        let mut visited: HashSet<(usize, usize, DIR)> = HashSet::new();
        visited.insert((pos_x, pos_y, dir));

        while !self.is_facing_bound(pos_x, pos_y, dir) {
            self.move_step(&lines, &mut pos_x, &mut pos_y, &mut dir);
            if visited.contains(&(pos_x, pos_y, dir)) {
                return true;
            }
            visited.insert((pos_x, pos_y, dir));
        }
        false
    }
}

impl Day for Day6 {
    fn part_1(&mut self) -> u64 {
        let mut pos_x = self.start_x;
        let mut pos_y = self.start_y;
        let mut dir = self.dir;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((pos_x, pos_y));
        let mut count = 1;

        while !self.is_facing_bound(pos_x, pos_y, dir) {
            self.move_step(&self.lines, &mut pos_x, &mut pos_y, &mut dir);
            if !visited.contains(&(pos_x, pos_y)) {
                count += 1;
                visited.insert((pos_x, pos_y));
            }
        }
        count
    }

    fn part_2(&mut self) -> u64 {
        let coords: Vec<(usize, usize)> = (0..self.lim_y)
            .flat_map(|y| (0..self.lim_x).map(move |x| (x, y)))
            .collect();

        coords
            .par_iter()
            .filter_map(|(x, y)| {
                if (*x == self.start_x && *y == self.start_y) || self.lines[*y][*x] == '#' {
                    return None;
                }
                let lines_clone = (*self.lines).clone();
                let is_loop = self.run_simulation(lines_clone, *x, *y);
                if is_loop {
                    Some(1)
                } else {
                    None
                }
            })
            .sum()
    }
}
