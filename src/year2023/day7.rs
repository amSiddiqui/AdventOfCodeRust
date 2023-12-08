use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;
use std::iter::zip;
use lazy_static::lazy_static;


lazy_static! {
    static ref CARD_MAPPING: HashMap<char, usize> = {
        let cards = "23456789TJQKA";
        let mut m = HashMap::with_capacity(cards.len());
        for (i, c) in cards.chars().enumerate() {
            m.insert(c, i);
        }
        m
    };
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: i32,
    frequencies: Vec<usize>
}

impl Hand {
    fn get_frequencies(hand: &str) -> HashMap<char, usize>{
        let mut freq = HashMap::new();
        for c in hand.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        freq
    }

    fn new(line: &str) -> Hand {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2);
        let mut f: Vec<usize> = Hand::get_frequencies(parts[0])
            .into_values()
            .collect();
        f.sort();
        f.reverse();
        Hand {
            hand: parts[0].to_string(),
            bid: parts[1].parse::<i32>().unwrap_or(0),
            frequencies: f
        }
    }

    fn get_score(&self) -> usize {
        let freq = &self.frequencies;
        if freq[0] == 5 {
            10
        } else if freq[0] == 4 {
            9
        } else if *freq == vec![3_usize, 2_usize] {
            8
        } else if freq[0] == 3 {
            7
        } else if freq[0..2] == vec![2_usize, 2_usize] {
            6
        } else if freq[0] == 2 {
            5
        } else if *freq == vec![1_usize; 5] {
            4
        } else {
            3
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Hand { }

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let score_self = self.get_score();
        let score_other = other.get_score();
        if score_self == score_other {
            for (a, b) in zip(self.hand.chars(), other.hand.chars()) {
                if a != b {
                    let val_a = CARD_MAPPING.get(&a)
                        .unwrap_or_else(|| panic!("Key {} not found", &a));
                    let val_b = CARD_MAPPING.get(&b)
                        .unwrap_or_else(|| panic!("Key {} not found", &a));
                    return val_a.cmp(val_b);
                }
            }
            Ordering::Equal
        } else {
            score_self.cmp(&score_other)
        }
    }
}


struct Day7 {
    input: Vec<Hand>
}

impl Day7 {
    fn new() -> Self {
        Day7 {
            input: Day7::parse_input()
        }
    }

    fn get_input() -> String {
        let data_path = "data/day7";
        fs::read_to_string(data_path).unwrap_or_default()
    }

    fn parse_input() -> Vec<Hand>{
        let data: Vec<Hand> = Day7::get_input()
            .split('\n')
            .map(Hand::new).collect();

        assert_eq!(data.len(), 1000);
        data
    }

    fn part1(&mut self) -> u64 {
        self.input.sort();
        let mut sum: u64 = 0;
        for (i, s) in self.input.iter().enumerate() {
           sum += (i as u64 + 1) * s.bid as u64;
        }
        sum
    }
}


#[cfg(test)]
mod test {
    use std::time::Instant;
    use crate::year2023::day7::Day7;

    #[test]
    fn test_part1() {
        let start = Instant::now();
        let mut day = Day7::new();
        let res = day.part1();
        let duration = start.elapsed();
        assert_eq!(res, 249638405);
        println!("Part 1 took {:?}", duration);
    }
}