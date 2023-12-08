use std::fs;
use std::iter::zip;
use crate::traits::Day;

pub struct Day6 {
    time_str: String,
    distance_str: String
}

impl Day6 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day6")
            .unwrap_or_else(|x| panic!("Empty input provided: {}", x));
        let lines:Vec<&str> = data.split('\n').collect();
        if lines.len() < 2 {
            panic!("Input file does not contain 2 lines");
        }
        let mut parts = lines[0].split(':');
        parts.next();
        let time_str = parts.next()
            .unwrap_or_else(|| panic!("Line 1 does not contain times"))
            .trim()
            .to_string();

        let mut parts = lines[1].split(':');
        parts.next();
        let distance_str = parts.next()
            .unwrap_or_else(|| panic!("Line 2 does not contain times"))
            .trim()
            .to_string();

        Day6 {
            time_str,
            distance_str
        }
    }
}


impl Day for Day6 {
    fn part_1(&self) -> u64 {
        let time_num: Vec<i32> = self.time_str
            .split_whitespace()
            .map(|x| x.parse::<i32>()
                    .unwrap_or_else(|err| panic!("Time values contains a non int string {};; err {}", x, err)))
            .collect();
        let distance_num: Vec<i32> = self.distance_str
            .split_whitespace()
            .map(|x| x.parse::<i32>()
                .unwrap_or_else(|err| panic!("Time values contains a non int string {};; err {}", x, err)))
            .collect();

        let mut res: u64 = 1;
        for (t, d) in zip(time_num, distance_num) {
            let mut count: u64 = 0;
            for i in 0..t {
                if i * (t - i) > d {
                    count += 1;
                }
            }
            res *= count;
        }

        res
    }

    fn part_2(&self) -> u64 {
        let time_str = self.time_str.split_whitespace().collect::<String>();
        let distance_str = self.distance_str.split_whitespace().collect::<String>();
        let time_num = time_str.parse::<u64>()
            .unwrap_or_else(|x| panic!("Cannot parse time {}", x));
        let distance_num = distance_str.parse::<u64>()
            .unwrap_or_else(|err| panic!("Cannot parse distance {}", err));

        let mut res: u64 = 0;
        for i in 0..time_num {
            if i * (time_num - i) > distance_num {
                res += 1;
            }
        }

        res
    }
}
