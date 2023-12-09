use std::fs;
use crate::traits::Day;

pub struct Day9 {
    sequences: Vec<Vec<i32>>
}

impl Day9 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day9")
            .unwrap_or_else(|err| panic!("Error reading file {}", err));
        let mut sequences = Vec::new();
        for d in data.split('\n') {
            let sequence: Vec<i32> = d.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap_or_else(|err| panic!("Cannot parse input, malformed value {};; err {}", x, err)))
                .collect();
            sequences.push(sequence);
        }

        Day9 {
            sequences
        }
    }

    fn get_sequence_last(seq: &Vec<i32>) -> i32 {
        let mut res: i32 = *seq.last().unwrap_or(&0);
        let mut curr_seq = seq.clone();
        while curr_seq.len() > 1 && curr_seq.iter().any(|x| *x != 0) {
            let mut next_seq = Vec::new();
            for i in 0..curr_seq.len() - 1 {
                next_seq.push(curr_seq[i + 1] - curr_seq[i]);
            }
            res += *next_seq.last().unwrap_or(&0);
            curr_seq = next_seq;
        }
        res
    }

    fn get_sequence_first(seq: &Vec<i32>) -> i32 {
        let mut states: Vec<Vec<i32>> = vec![seq.clone()];
        let mut curr_seq = seq.clone();
        while curr_seq.len() > 1 && curr_seq.iter().any(|x| *x != 0) {
            let mut next_seq = Vec::new();
            for i in 0..curr_seq.len() - 1 {
                next_seq.push(curr_seq[i + 1] - curr_seq[i]);
            }
            states.push(next_seq.clone());
            curr_seq = next_seq;
        }
        let n = states.len();
        states[n - 1].insert(0, 0);
        for i in (1..n).rev() {
            let res = states[i-1][0] - states[i][0];
            states[i-1].insert(0, res);
        }
        states[0][0]
    }
}

impl Day for Day9 {
    fn part_1(&self) -> u64 {
        let mut res = 0;
        for seq in &self.sequences {
            res += Day9::get_sequence_last(seq)
        }
        res as u64
    }

    fn part_2(&self) -> u64 {
        let mut res = 0;
        for seq in &self.sequences {
            res += Day9::get_sequence_first(seq);
        }
        res as u64
    }
}
