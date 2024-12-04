use std::fs;

use crate::traits::Day;

pub struct Day4 {
    lines: Vec<Vec<char>>,
    x_lim: i32,
    y_lim: i32,
}

impl Day4 {
    pub fn new() -> Self {
        let lines: Vec<Vec<char>> = fs::read_to_string("data/year2024/day4")
            .expect("cannot read file")
            .split('\n')
            .map(|line| line.chars().collect())
            .collect();
        let x_lim = (&lines)[0].len() as i32;
        let y_lim = (&lines).len() as i32;

        Day4 { lines, x_lim, y_lim }
    }

    pub fn search(&self, x: i32, y: i32, step_x: i32, step_y: i32, haystack: &str) -> u64 {
        let mut start_x = 0;
        let mut start_y = 0;
        for c in haystack.chars() {
            let y_pos = y + start_y;
            let x_pos = x + start_x;
            if y_pos < 0 {
                return 0;
            }
            if y_pos >= self.y_lim {
                return 0;
            }
            if x_pos < 0 {
                return 0;
            }
            if x_pos >= self.x_lim {
                return 0;
            }
            if (&self.lines)[y_pos as usize][x_pos  as usize] != c {
                return 0;
            }
            start_x += step_x;
            start_y += step_y;
        }
        
        1
    }
}

impl Day for Day4 {
    fn part_1(&mut self) -> u64 {
        let mut count = 0;
        let haystack = "XMAS";
        for y_i in 0..(&self.lines).len() {
            for x_i in 0..(&self.lines)[y_i].len() {
                let x = x_i as i32;
                let y = y_i as i32;
                let total = self.search(x, y, 1, 0, haystack) +
                self.search(x, y, -1, 0, haystack) +
                self.search(x, y, 0, 1, haystack) +
                self.search(x, y, 0, -1, haystack) + 
                self.search(x, y, 1, 1, haystack) +
                self.search(x, y, 1, -1, haystack) +
                self.search(x, y, -1, 1, haystack) +
                self.search(x, y, -1, -1, haystack);
                count += total;
            }
        }
        count
    }

    fn part_2(&mut self) -> u64 {
        let mut count = 0;
        for y_i in 0..(&self.lines).len() {
            for x_i in 0..(&self.lines)[y_i].len() {
                let x = x_i as i32;
                let y = y_i as i32;
                if (self.search(x, y, 1, 1, "MAS") == 1 || self.search(x, y, 1, 1, "SAM") == 1) 
                && (self.search(x, y+2, 1, -1, "MAS") == 1 || self.search(x, y+2, 1, -1, "SAM") == 1) {
                    count += 1;
                }
            }
        }
        count
    }
}
