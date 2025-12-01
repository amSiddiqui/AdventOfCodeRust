use crate::traits::Day;
use std::fs;

pub struct Day1 {
    data: Vec<(bool, i32)>,
}

impl Day1 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day1")
            .expect("Data file cannot be read")
            .lines()
            .enumerate()
            .map(|(n, line)| {
                let dir = line.chars().next().expect("Char not found");
                let num_str = &line[1..];
                (dir == 'L', i32::from_str_radix(num_str, 10).expect(&format!("Line {} cannot be read", n + 1)))
            })
            .collect();

        Day1 { data }
    }
}

impl Day for Day1 {
    fn part_1(&mut self) -> u64 {
        let mut res: i32 = 50;
        let mut count = 0;
        self.data.iter().for_each(|(l, num)| {
            if *l {
                res = res - num;
            } else {
                res += num;
            }
            while res < 0 {
                res = 100 + res;
            } 
            while res >= 100 {
                res = res - 100;
            }

            if res == 0 {
                count = count + 1;
            }
        });
        count
    }

    fn part_2(&mut self) -> u64 {
        let mut res: i32 = 50;
        let mut count = 0;
        self.data.iter().for_each(|(l, num)| {
            if *l {
                for _ in 0..*num {
                    res -= 1;
                    if res == 0 {
                        count += 1;
                    } 
                    if res < 0 {
                        res = 99;
                    }
                }
            } else {
                for _ in 0..*num {
                    res += 1;
                    if res == 100 {
                        count += 1;
                        res = 0;
                    }
                }
            }
        });
        count
    }
}
