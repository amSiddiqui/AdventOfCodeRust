use std::collections::HashMap;
use std::fs;
use crate::traits::Day;
use rayon::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WORD_DIGIT_MAP: HashMap<String, u32> = {
        let mut word_map = HashMap::new();
        word_map.insert(String::from("one"), 1);
        word_map.insert(String::from("two"), 2);
        word_map.insert(String::from("three"), 3);
        word_map.insert(String::from("four"), 4);
        word_map.insert(String::from("five"), 5);
        word_map.insert(String::from("six"), 6);
        word_map.insert(String::from("seven"), 7);
        word_map.insert(String::from("eight"), 8);
        word_map.insert(String::from("nine"), 9);
        word_map
    };
}


pub struct Day1 {
    lines: Vec<String>,
    digit_re: Regex,
}

impl Day1 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day1")
            .expect("Cannot read file")
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>();

        let digit_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").expect("Invalid Regex");
        Day1 {
            lines,
            digit_re
        }
    }

    pub fn first_digit(line: &str) -> u32 {
        let first_digit = line.chars().find(|x| x.is_ascii_digit()).expect("Line does not have a digit");
        first_digit.to_digit(10).unwrap()
    }

    pub fn last_digit(line: &str) -> u32 {
        let last_digit = line.chars().filter(|x| x.is_ascii_digit()).last().expect("Line does not have a digit");
        last_digit.to_digit(10).unwrap()
    }
}

impl Day for Day1 {
    fn part_1(&mut self) -> u64 {
        self.lines.par_iter().map(|line| {
            Day1::first_digit(line) * 10 + Day1::last_digit(line)
        }).sum::<u32>() as u64
    }

    fn part_2(&mut self) -> u64 {
        let res = self.lines.par_iter().map(|x| {
            let mut new_line = x.clone();
            for (key, val) in WORD_DIGIT_MAP.iter() {
                new_line = new_line.replace(&val.to_string(), key);
            }
            let first_digit = self.digit_re.find_iter(&new_line)
                .next()
                .expect("No digits found")
                .as_str();
            let f_val = *WORD_DIGIT_MAP
                .get(first_digit)
                .expect("Digit mapping not found");
            new_line = new_line.replacen(first_digit, &f_val.to_string(), 1);
            let mut largest_digit = None;
            let mut largest_index = 0;
            for (key, &val) in WORD_DIGIT_MAP.iter() {
                if let Some(index) = new_line.rfind(key) {
                    if index > largest_index {
                        largest_digit = Some(val);
                        largest_index = index;
                    }
                }
            }
            if let Some(digit) = largest_digit {
                f_val * 10 + digit
            } else {
                f_val * 10 + f_val
            }
        }).sum::<u32>();
        res as u64
    }
}


#[cfg(test)]
mod tests {
    use crate::year2023::day1::Day1;

    #[test]
    fn test_regex() {
        let day = Day1::new();
        for s in day.digit_re.find_iter("somethingonetwothreeight") {
            println!("{}", s.as_str());
        }
    }
}
