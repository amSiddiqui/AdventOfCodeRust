use std::collections::HashMap;
use std::fs;
use crate::traits::Day;
use rayon::prelude::*;

pub struct Day13 {
    data: Vec<Vec<Vec<u8>>>
}

impl Day13 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2023/day13")
            .expect("Cannot read data")
            .split("\n\n")
            .map(|land| land
                .split('\n')
                .map(|line| line
                    .chars()
                    .map(|x| if x == '#' { 1_u8 } else { 0_u8 })
                    .collect::<Vec<_>>()
                ).collect::<Vec<_>>()
            ).collect::<Vec<_>>();

        Day13 {
            data
        }
    }

    fn transpose(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        if matrix.is_empty() {
            return Vec::new();
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut res = vec![vec![0_u8; rows]; cols];
        for (r, row) in matrix.iter().enumerate() {
            for (c, &col) in row.iter().enumerate() {
                res[c][r] = col;
            }
        }
        res
    }

    fn is_reflected(matrix: &Vec<Vec<u8>>, i: usize) -> bool {
        if i == 0 {
            return false;
        }
        let limit = i.min(matrix.len() - i);
        let mut count = 0;
        let mut x = i-1;
        let mut y = i;
        while count < limit {
            if matrix[x] != matrix[y] {
                return false;
            }
            x = x.saturating_sub(1);
            y += 1;
            count += 1;
        }
        true
    }

    fn find_reflection_index(matrix: &Vec<Vec<u8>>) -> Option<usize> {
        for i in 1..matrix.len() {
            if Day13::is_reflected(matrix, i) {
                return Some(100 * i);
            }
        }

        let matrix_t = Day13::transpose(matrix);
        (1..matrix_t.len()).find(|&i| Day13::is_reflected(&matrix_t, i))
    }
}

impl Day for Day13 {
    fn part_1(&mut self) -> u64 {
        let res = self.data.par_iter()
            .filter_map(Day13::find_reflection_index)
            .sum::<usize>();
        res as u64
    }

    fn part_2(&mut self) -> u64 {
        let res = self.data.par_iter().map(|matrix| {
            let mut ans_count = HashMap::new();
            for (r, row) in matrix.iter().enumerate() {
                for (c, val) in row.iter().enumerate() {
                    let mut mat = matrix.clone();
                    mat[r][c] = if *val == 1 {
                        0
                    } else {
                        1
                    };
                    for i in 1..mat.len() {
                        if Day13::is_reflected(&mat, i) {
                             let idx = 100 * i;
                            *ans_count.entry(idx).or_insert(0) += 1;
                        }
                    }

                    let matrix_t = Day13::transpose(&mat);
                    for i in 1..matrix_t.len() {
                        if Day13::is_reflected(&matrix_t, i) {
                            let idx = i;
                            *ans_count.entry(idx).or_insert(0) += 1;
                        }
                    }
                }
            }
            let mut min_val = i32::MAX;
            let mut min_idx = 0;
            for (idx, count) in ans_count.iter() {
                if *count < min_val {
                    min_idx = *idx;
                    min_val = *count;
                }
            }
            min_idx
        }).sum::<usize>();
        res as u64
    }
}

