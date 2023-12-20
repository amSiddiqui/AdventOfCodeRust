use std::collections::HashMap;
use std::fs;
use crate::traits::Day;

pub struct Day14 {
    lines: Vec<Vec<char>>,
}

impl Day14 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day14")
            .expect("Cannot read from file data/day14")
            .split('\n')
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Day14 {
            lines
        }
    }

    fn move_north(lines: &Vec<Vec<char>>)  -> Vec<Vec<char>> {
        let mut rocks = lines.clone();
        let mut av_x = vec![0; lines[0].len()];
        for (x, row) in lines.iter().enumerate() {
            for (y, &rock) in row.iter().enumerate() {
                if rock == '.' {
                    continue;
                } else if rock == '#' {
                    av_x[y] = x + 1;
                } else {
                    rocks[av_x[y]][y] = '0';
                    if av_x[y] != x {
                        rocks[x][y] = '.';
                    }
                    av_x[y] += 1;
                }
            }
        }
        rocks
    }

    fn move_south(lines: &Vec<Vec<char>>)  -> Vec<Vec<char>> {
        let mut rocks = lines.clone();
        let mut av_x = vec![lines.len() - 1; lines[0].len()];
        for x in (0..lines.len()).rev() {
            for y in (0..lines[x].len()).rev() {
                let rock = lines[x][y];
                if rock == '.' {
                    continue;
                } else if rock == '#' {
                    av_x[y] = x - 1;
                } else {
                    rocks[av_x[y]][y] = '0';
                    if av_x[y] != x {
                        rocks[x][y] = '.';
                    }
                    av_x[y] -= 1;
                }
            }
        }
        rocks
    }

    fn move_west(lines: &Vec<Vec<char>>)  -> Vec<Vec<char>> {
        let mut rocks = lines.clone();
        let mut av_y = vec![0; lines.len()];
        for (x, row) in lines.iter().enumerate() {
            for (y, &rock) in row.iter().enumerate() {
                if rock == '.' {
                    continue;
                } else if rock == '#' {
                    av_y[x] = y + 1;
                } else {
                    rocks[x][av_y[x]] = '0';
                    if av_y[x] != y {
                        rocks[x][y] = '.';
                    }
                    av_y[x] += 1;
                }
            }
        }
        rocks
    }

    fn move_east(lines: &Vec<Vec<char>>)  -> Vec<Vec<char>> {
        let mut rocks = lines.clone();
        let mut av_y = vec![lines[0].len() - 1; lines.len()];
        for x in (0..lines.len()).rev() {
            for y in (0..lines[x].len()).rev() {
                let rock = lines[x][y];
                if rock == '.' {
                    continue;
                } else if rock == '#' {
                    av_y[x] = y - 1;
                } else {
                    rocks[x][av_y[x]] = '0';
                    if av_y[x] != y {
                        rocks[x][y] = '.';
                    }
                    av_y[x] -= 1;
                }
            }
        }
        rocks
    }

    fn calc_sum(rocks: Vec<Vec<char>>) -> u64 {
        let mut res = 0;
        let n = rocks.len();
        for (x, row) in rocks.iter().enumerate() {
            for rock in row {
                if *rock == '0' {
                    res += n - x;
                }
            }
        }
        res as u64
    }

    fn one_cycle(start: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let res = Day14::move_north(start);
        let res = Day14::move_west(&res);
        let res = Day14::move_south(&res);
        Day14::move_east(&res)
    }
}

impl Day for Day14 {
    fn part_1(&mut self) -> u64 {
        let north_rocks = Day14::move_north(&self.lines);
        Day14::calc_sum(north_rocks)
    }

    fn part_2(&mut self) -> u64 {
        let mut res = self.lines.clone();
        let mut cache = HashMap::new();
        let mut cycle_start = 0;
        let mut cycle_end=  0;
        let mut found = false;
        cache.insert(res.clone(), 0);
        let limit = 1000000000;
        for i in 0..limit {
            res = Day14::one_cycle(&res);
            if let Some(j) = cache.get(&res) {
                cycle_end = i;
                cycle_start = *j;
                found = true;
                break;
            }
            cache.insert(res.clone(), i+1);
        }
        if !found {
            panic!("No cycles exists");
        }

        let nth = cycle_start + ((limit - cycle_start) % (cycle_end - cycle_start + 1));
        res = self.lines.clone();
        for _ in 0..nth {
            res = Day14::one_cycle(&res);
        }
        Day14::calc_sum(res)
    }
}


