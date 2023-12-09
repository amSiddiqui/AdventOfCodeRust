use std::fs;
use crate::traits::Day;
use regex::Regex;
use rayon::prelude::*;

pub struct Day3 {
    lines: Vec<String>,
    num_re: Regex,
    symbol_re: Regex
}

impl Day3 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day3")
            .unwrap_or_else(|err| panic!("Cannot read input;; Err {err}"))
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>();

        Day3 {
            lines,
            num_re: Regex::new(r"\d+").unwrap(),
            symbol_re: Regex::new(r"[^0-9.]").unwrap()
        }
    }

    fn check_if_symbol_present(&self, x: usize, mut y1: usize, mut y2: usize) -> bool {
        // Check same line
        if y1 > 0 &&
            self.symbol_re.is_match(&self.lines[x][y1-1..y1]) {
            return true
        }
        if y2 < self.lines[x].len() - 1 &&
            self.symbol_re.is_match(&self.lines[x][y2+1..y2+2]) {
            return true
        }
        // Check line above
        if y1 > 0 {
            y1 = y1.saturating_sub(1);
        }
        if y2 < self.lines[x].len() - 1 {
            y2 += 1;
        }
        if x > 0 && self.symbol_re.is_match(&self.lines[x-1][y1..y2+1]) {
            return true
        }
        // Check line below
        if x < self.lines.len() -1 &&
            self.symbol_re.is_match(&self.lines[x+1][y1..y2+1]) {
            return true
        }
        false
    }

    fn find_adjacent_star(&self, x: usize, y: usize) -> Vec<u64> {
        let mut adjacent = Vec::new();
        if x > 0 {
            for num in self.num_re.find_iter(&self.lines[x-1]) {
                let y1 = num.start();
                let y2 = num.end() - 1;
                if !(y2 < y - 1 || y1 > y + 1) {
                    adjacent.push(num.as_str().parse::<u64>().unwrap());
                }
            }
        }

        for num in self.num_re.find_iter(&self.lines[x]) {
            let y1 = num.start();
            let y2 = num.end() - 1;
            if y1 == y + 1 || y2 == y - 1 {
                adjacent.push(num.as_str().parse::<u64>().unwrap());
            }
        }

        if x < self.lines.len() - 1 {
            for num in self.num_re.find_iter(&self.lines[x+1]) {
                let y1 = num.start();
                let y2 = num.end() - 1;
                if !(y2 < y - 1 || y1 > y + 1) {
                    adjacent.push(num.as_str().parse::<u64>().unwrap());
                }
            }
        }

        adjacent
    }
}

impl Day for Day3 {
    fn part_1(&self) -> u64 {
        let res: u64 = self.lines.par_iter()
            .enumerate()
            .map(|(x, l)| {
                self.num_re.find_iter(l)
                    .filter_map(|n| {
                        let num = n.as_str().parse::<u64>().unwrap();
                        if self.check_if_symbol_present(x, n.start(), n.end() - 1) {
                            Some(num)
                        } else {
                            None
                        }
                    }).sum::<u64>()
            }).sum();
        res
    }

    fn part_2(&self) -> u64 {
        let res: u64 = self.lines.par_iter()
            .enumerate()
            .map(|(x, line)| {
                line.char_indices()
                    .filter_map(|(y, c)| if c == '*' { Some(y) } else { None })
                    .filter_map(|y| {
                        let res = self.find_adjacent_star(x, y);
                        if res.len() == 2 {
                            Some(res[0] * res[1])
                        } else {
                            None
                        }
                    }).sum::<u64>()
            }).sum();
        res
    }
}


#[cfg(test)]
mod tests {
    use crate::year2023::day3::Day3;

    #[test]
    fn test_regex() {
        let day = Day3::new();
        assert!(day.num_re.is_match("somethign 123 245 something"));
        assert!(day.symbol_re.is_match("...&-+"));

        for num in day.num_re.find_iter("somethign 123 245 something") {
            println!("{}", num.as_str());
        }
    }
}
