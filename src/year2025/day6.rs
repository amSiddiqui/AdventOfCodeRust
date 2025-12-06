use crate::traits::Day;
use std::fs;

pub struct Day6 {
    numbers: Vec<Vec<u64>>,
    symbols: Vec<char>
}

impl Day6 {
    pub fn new() -> Self {
        let data =fs::read_to_string("data/year2025/day6")
            .expect("Cannot read data"); 
        let mut all_lines = data
            .lines()
            .rev();
        let last_line = all_lines.next().expect("last line not found");
        let symbols = last_line.trim().split(" ")
        .filter(|p| p.len() > 0)
        .map(|p| p.chars().next().unwrap())
        .filter(|p| *p != ' ')
        .collect();
        let numbers = all_lines
            .map(|line| {
                line.
                trim()
                .replace("  ", " ")
                .split(" ")
                .filter(|n| n.trim().len() > 0)
                .map(|num| u64::from_str_radix(num.trim(), 10).unwrap()).collect()
            }).collect();
        Day6 { numbers, symbols }
    }
}

impl Day for Day6 {
    fn part_1(&mut self) -> u64 {
        let mut solution = vec![];
        for sym in &self.symbols { 
            if *sym == '*' {
                solution.push(1u64);
            } else {
                solution.push(0u64);
            }
        }
        for num in &self.numbers {
            for i in 0..self.symbols.len() {
                if self.symbols[i] == '*' {
                    solution[i] *= num[i];
                } else {
                    solution[i] += num[i];
                }
            }
        }
        solution.iter().sum()
    }
    fn part_2(&mut self) -> u64 {
        let data =fs::read_to_string("data/year2025/day6")
            .expect("Cannot read data"); 
        let mut symbols: Vec<char> = vec![];
        let mut column_width: Vec<usize> = vec![];
        let mut all_lines = data.lines().rev();
        let mut sym_line = all_lines.next().unwrap().chars();
        symbols.push(sym_line.next().unwrap());
        let mut w = 0;
        for ch in sym_line {
            if ch == '*' || ch == '+' {
                symbols.push(ch);
                column_width.push(w);
                w = 0;
            } else {
                w += 1;
            }
        }
        column_width.push(w+1);
        let mut all_nums = vec![];
        for lin in all_lines {
            let mut i = 0;
            let mut line_nums = vec![];
            for w in &column_width {
                line_nums.push(&lin[i..(i+w)]);
                i = i+w+1;
            }
            all_nums.push(line_nums);
        }
        let mut res = 0;
        for (i, sym) in symbols.iter().enumerate() {
            let mut new_nums: Vec<u64> = vec![0; column_width[i]];
            for nums in all_nums.iter().rev() {
                let col = nums[i];
                for (j, ch) in col.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }
                    new_nums[j] = new_nums[j] * 10 + ch.to_digit(10).unwrap() as u64;
                }
            }
            if *sym == '*' {
                let mut acc = 1;
                new_nums.iter().for_each(|n| {
                    acc *= n;
                });
                res += acc;
            }
            if *sym == '+' {
                let mut acc = 0;
                new_nums.iter().for_each(|n| {
                    acc += n;
                });
                res += acc;
            }
            
        }

        res
    }
}
