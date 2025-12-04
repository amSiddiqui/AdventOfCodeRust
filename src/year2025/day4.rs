use crate::traits::Day;
use std::fs;

pub struct Day4 {
    data: Vec<Vec<char>>,
}

impl Day4 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day4")
            .expect("Cannot read data")
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Day4 { data }
    }
}

impl Day for Day4 {
    fn part_1(&mut self) -> u64 {
        let mut count = 0;
        let y_lim = self.data.len();
        let x_lim = self.data[0].len();
        self.data.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, val)| {
                if *val != '@' {
                    return;
                }
                let mut ct = 0;
                // top row
                if y > 0 && x > 0 && self.data[y - 1][x - 1] == '@' {
                    ct += 1;
                }
                if y > 0 && self.data[y - 1][x] == '@' {
                    ct += 1;
                }
                if y > 0 && x < x_lim - 1 && self.data[y - 1][x + 1] == '@' {
                    ct += 1;
                }

                // bottom row
                if y < y_lim - 1 && x > 0 && self.data[y + 1][x - 1] == '@' {
                    ct += 1;
                }
                if y < y_lim - 1 && self.data[y + 1][x] == '@' {
                    ct += 1;
                }
                if y < y_lim - 1 && x < x_lim - 1 && self.data[y + 1][x + 1] == '@' {
                    ct += 1;
                }

                // same row
                if x > 0 && self.data[y][x - 1] == '@' {
                    ct += 1;
                }
                if x < x_lim - 1 && self.data[y][x + 1] == '@' {
                    ct += 1;
                }

                if ct < 4 {
                    count += 1;
                }
            });
        });
        count
    }
    fn part_2(&mut self) -> u64 {
        let mut count = 0;
        let mut count_total = 0;
        let y_lim = self.data.len();
        let x_lim = self.data[0].len();
        loop {
            let mut i_rem: Vec<(usize, usize)> = vec![];

            self.data.iter().enumerate().for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, val)| {
                    if *val != '@' {
                        return;
                    }
                    let mut ct = 0;
                    // top row
                    if y > 0 && x > 0 && self.data[y - 1][x - 1] == '@' {
                        ct += 1;
                    }
                    if y > 0 && self.data[y - 1][x] == '@' {
                        ct += 1;
                    }
                    if y > 0 && x < x_lim - 1 && self.data[y - 1][x + 1] == '@' {
                        ct += 1;
                    }

                    // bottom row
                    if y < y_lim - 1 && x > 0 && self.data[y + 1][x - 1] == '@' {
                        ct += 1;
                    }
                    if y < y_lim - 1 && self.data[y + 1][x] == '@' {
                        ct += 1;
                    }
                    if y < y_lim - 1 && x < x_lim - 1 && self.data[y + 1][x + 1] == '@' {
                        ct += 1;
                    }

                    // same row
                    if x > 0 && self.data[y][x - 1] == '@' {
                        ct += 1;
                    }
                    if x < x_lim - 1 && self.data[y][x + 1] == '@' {
                        ct += 1;
                    }

                    if ct < 4 {
                        count += 1;
                        i_rem.push((x, y))
                    }
                });
            });
            for (x, y) in i_rem {
                self.data[y][x] = '.';
            }

            if count == 0 {
                break;
            }
            count_total += count;
            dbg!(count);
            count = 0;
        }
        count_total
    }
}
