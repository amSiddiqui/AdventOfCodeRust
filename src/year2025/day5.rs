use crate::traits::Day;
use std::fs;

pub struct Day5 {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>
}

impl Day5 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day5")
            .expect("Cannot read data");
        let mut parts = data.split("\n\n");
        let ranges = parts.next().unwrap().split("\n").map(|line| {
            let mut pts = line.split("-");
            let start = u64::from_str_radix(pts.next().unwrap(), 10).unwrap();
            let end = u64::from_str_radix(pts.next().unwrap(), 10).unwrap();
            (start, end)
        }).collect();

        let ids = parts.next().unwrap().split("\n").map(|line| {
            u64::from_str_radix(line, 10).unwrap()
        }).collect();

        Day5 { ranges, ids }
    }
}

impl Day for Day5 {
    fn part_1(&mut self) -> u64 {
        let mut count = 0;
        for id in &self.ids {
            for (st, ed) in &self.ranges {
                if id >= st && id <= ed {
                    count += 1;
                    // dbg!(id);
                    break;
                }
            }
        }
        count
    }
    fn part_2(&mut self) -> u64 {
        self.ranges.sort_by(|a, b| {
           a.0.cmp(&b.0)
        });
        let l = self.ranges.len();
        let mut j = 1;

        let mut cmb_ranges = vec![];
        let mut current_range = self.ranges[0];
        while j < l {
            let b = self.ranges[j];
            if b.0 > current_range.1 {
                cmb_ranges.push(current_range);
                current_range = b;
                j+=1;
                continue;
            }
            if b.1 <= current_range.1 {
                j += 1;
                continue;
            }
            current_range.1 = b.1;
            j += 1;
        }
        cmb_ranges.push(current_range);
        let mut sum = 0;
        for rng in cmb_ranges {
            sum += 1 + rng.1 - rng.0;
        }
        sum
    }
}
