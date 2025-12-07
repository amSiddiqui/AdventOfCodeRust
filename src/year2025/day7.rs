use ahash::AHashMap;

use crate::traits::Day;
use std::{fs};

pub struct Day7 {
    data: Vec<Vec<char>>,
}

impl Day7 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day7")
            .expect("Cannot read data")
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Day7 { data }
    }
}


fn tick2(data: &mut Vec<Vec<char>>, x: usize, y: usize, cache: &mut AHashMap<(usize, usize), u64>) -> u64 {
    let y_lim = data.len();
    let x_lim = data[0].len();
    if let Some(val) = cache.get(&(x, y)) {
        return *val;
    }

    if y == y_lim - 1 {
        return 1;
    }
    if data[y+1][x] == '.' || data[y+1][x] == '|' {
        data[y+1][x] = '|';
        let res = tick2(data, x, y+1, cache);
        cache.insert((x, y+1), res);
        return res;
    }
    if data[y+1][x] == '^' {
        let left = if x == 0 { x } else { x-1 };
        let right = if x == x_lim - 1 { x } else { x+1 };
        let mut a = 0;
        let mut b = 0;
        if data[y+1][left] != '^' {
            data[y+1][left] = '|';
            a = tick2(data,left, y+1, cache);
            cache.insert((left, y+1), a);
        }
        if data[y+1][right] != '^' {
            data[y+1][right] = '|';
            b = tick2(data, right, y+1, cache);
            cache.insert((right, y+1), b);
        }
        return a + b;
    }
    return 0;
}

fn tick(data: &mut Vec<Vec<char>>, x: usize, y: usize, count: &mut u64) {
    let y_lim = data.len();
    let x_lim = data[0].len();

    if y == y_lim - 1 {
        return;
    }
    if data[y+1][x] == '.' {
        data[y+1][x] = '|';
        return tick(data, x, y+1, count);
    }
    if data[y+1][x] == '^' {
        let left = if x == 0 { x } else { x-1 };
        let right = if x == x_lim - 1 { x } else { x+1 };
        if data[y+1][left] != '^' {
            data[y+1][left] = '|';
        }
        if data[y+1][right] != '^' {
            data[y+1][right] = '|';
        }
        *count += 1;
        tick(data,left, y+1, count);
        tick(data, right, y+1, count);
    }
}

impl Day for Day7 {
    fn part_1(&mut self) -> u64 {
        let mut start = (0, 0);
        for (y, line) in self.data.iter().enumerate() {
            let mut found = false;
            for (x, ch) in line.iter().enumerate() {
                if *ch == 'S' {
                    start = (x, y);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        let mut count = 0;
        self.data[start.1+1][start.0] = '|';
        tick(&mut self.data, start.0, start.1 + 1, &mut count);

        // for lin in &self.data {
        //     for ch in lin {
        //         print!("{ch}");
        //     }
        //     println!();
        // }

        count
    }
    fn part_2(&mut self) -> u64 {
        let mut start = (0, 0);
        for (y, line) in self.data.iter().enumerate() {
            let mut found = false;
            for (x, ch) in line.iter().enumerate() {
                if *ch == 'S' {
                    start = (x, y);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        self.data[start.1+1][start.0] = '|';
        let mut cache: AHashMap<(usize, usize), u64> = AHashMap::new();
        return tick2(&mut self.data, start.0, start.1 + 1, &mut cache);
        
    }
}
