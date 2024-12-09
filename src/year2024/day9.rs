use std::fs;

use crate::traits::Day;

pub struct Day9 {
    input: Vec<u8>,
}

impl Day9 {
    pub fn new() -> Self {
        let input = fs::read_to_string("data/year2024/day9")
            .expect("cannot read data file")
            .chars()
            .map(|c| c.to_digit(10).expect("Not a digit") as u8)
            .collect();

        Day9 { input }
    }
}

fn find_multiply_sum(start_i: usize, end_i: usize, digit2: usize) -> u64 {
    let digit = digit2 / 2;
    // println!("Start: {}, End: {}, digit: {}", start_i, start_i + end_i, digit);
    let mut total = 0;
    for i in start_i..(end_i + start_i) {
        total += i * digit;
    }

    total as u64
}

impl Day for Day9 {
    fn part_1(&mut self) -> u64 {
        let mut i = 0;
        let mut j = self.input.len() - 1;
        if j % 2 == 1 {
            j -= 1;
        }

        // println!("Input: {:?}", self.input);

        let mut idx_sum = 0;
        let mut res = 0;
        while i <= j {
            // println!("i={}, j={}, input={:?}", i, j, self.input);
            if i % 2 == 0 {
                res += find_multiply_sum(idx_sum, self.input[i] as usize, i);
                idx_sum += self.input[i] as usize;
                i += 1;
                continue;
            }

            // handle empty space

            if self.input[i] > self.input[j] {
                res += find_multiply_sum(idx_sum, self.input[j] as usize, j);
                idx_sum += self.input[j] as usize;
                self.input[i] -= self.input[j];
                self.input[j] = 0;
                j -= 2;
                continue;
            }

            if self.input[i] < self.input[j] {
                res += find_multiply_sum(idx_sum, self.input[i] as usize, j);
                idx_sum += self.input[i] as usize;
                self.input[j] -= self.input[i];
                self.input[i] = 0;
                i += 1;
                continue;
            }

            if self.input[i] == self.input[j] {
                res += find_multiply_sum(idx_sum, self.input[i] as usize, j);
                idx_sum += self.input[i] as usize;
                self.input[i] = 0;
                self.input[j] = 0;
                i += 1;
                j -= 2;
                continue;
            }
        }

        //println!("i={}, j={} Final: {:?}", i, j, self.input);
        res as u64
    }

    fn part_2(&mut self) -> u64 {
        let mut i = 0;
        let mut res = 0;
        let mut idx_sum = 0;
        let input_cp = self.input.clone();

        while i < self.input.len() {
            if i % 2 == 0 {
                if self.input[i] != 0 {
                    res += find_multiply_sum(idx_sum, self.input[i] as usize, i);
                }
                idx_sum += input_cp[i] as usize;
                i += 1;
                // println!("i={}, Filled Space: {:?}", i, self.input);
                continue;
            }

            // handle free space
            let mut j = self.input.len() - 1;
            if j % 2 == 1 {
                j -= 1;
            }
            // find block that fits
            
            while j > i && self.input[i] > 0 {
                if self.input[j] <= self.input[i] && self.input[j] != 0 {
                    res += find_multiply_sum(idx_sum, self.input[j] as usize, j);
                    idx_sum += input_cp[j] as usize;
                    self.input[i] -= self.input[j];
                    self.input[j] = 0;
                    // println!("i={}, j={}, Empty Space: {:?}", i, j, self.input);
                }
                j -= 2;
            }
            idx_sum += self.input[i] as usize;
            i += 1;
        }

        res as u64
    }
}
