use std::cmp::Ordering;
use std::fs;
use std::collections::HashMap;
use std::iter::zip;
use lazy_static::lazy_static;
use crate::traits::Day;


lazy_static! {
    static ref CARD_MAPPING: HashMap<char, usize> = {
        let cards = "23456789TJQKA";
        let mut m = HashMap::with_capacity(cards.len());
        for (i, c) in cards.chars().enumerate() {
            m.insert(c, i);
        }
        m
    };

    static ref CARD_MAPPING_2: HashMap<char, usize> = {
        let cards = "J23456789TQKA";
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
    frequencies: Vec<usize>,
    joker: bool
}

impl Hand {
    fn get_frequencies(hand: &str) -> HashMap<char, usize>{
        let mut freq = HashMap::new();
        for c in hand.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        freq
    }

    fn largest_key(map: &HashMap<char, usize>) -> Option<&char> {
        let mut map_keys = map.keys().filter(|x| **x != 'J');
        if let Some(key) = map_keys.next() {
            let mut large = key;
            for k in map_keys {
                if map.get(k).unwrap() > map.get(large).unwrap() {
                    large = k;
                }
            }
            Some(large)
        } else {
            None
        }

    }

    fn new(line: &str, joker: bool) -> Hand {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 2, "Each line should have 2 part");
        let mut freq_map = Hand::get_frequencies(parts[0]);
        if joker {
            if let Some(&joker_val) = freq_map.get(&'J') {
                if let Some(k) = Hand::largest_key(&freq_map) {
                    freq_map.entry(*k)
                        .and_modify(|x| *x += joker_val);
                }
                freq_map.remove(&'J');
            }
        }
        let mut f: Vec<usize> = freq_map
            .into_values()
            .collect();
        f.sort();
        f.reverse();
        if f.is_empty() {
            f.push(5);
        }
        Hand {
            hand: parts[0].to_string(),
            bid: parts[1].parse::<i32>().unwrap_or(0),
            frequencies: f,
            joker
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
                    return if self.joker {
                        let val_a = CARD_MAPPING_2.get(&a)
                            .unwrap_or_else(|| panic!("Key {} not found", &a));
                        let val_b = CARD_MAPPING_2.get(&b)
                            .unwrap_or_else(|| panic!("Key {} not found", &a));
                        val_a.cmp(val_b)
                    } else {
                        let val_a = CARD_MAPPING.get(&a)
                            .unwrap_or_else(|| panic!("Key {} not found", &a));
                        let val_b = CARD_MAPPING.get(&b)
                            .unwrap_or_else(|| panic!("Key {} not found", &a));
                        val_a.cmp(val_b)
                    }
                }
            }
            Ordering::Equal
        } else {
            score_self.cmp(&score_other)
        }
    }
}


pub struct Day7 {
    input: String
}

impl Day7 {
    pub fn new() -> Self {
        Day7 {
            input: Day7::get_input()
        }
    }

    fn get_input() -> String {
        let data_path = "data/day7";
        fs::read_to_string(data_path).unwrap_or_default()
    }

    fn parse_input(&self, joker:bool) -> Vec<Hand>{
        let mut data: Vec<Hand> = self.input
            .split('\n')
            .map(|x| Hand::new(x, joker)).collect();

        assert_eq!(data.len(), 1000, "Input should have 1000 lines");
        data.sort();
        data
    }
}

impl Day for Day7 {
    fn part_1(&mut self) -> u64 {
        let mut sum: u64 = 0;
        let data = self.parse_input(false);
        for (i, s) in data.iter().enumerate() {
            sum += (i as u64 + 1) * s.bid as u64;
        }
        sum
    }

    fn part_2(&mut self) -> u64 {
        let mut sum: u64 = 0;
        let data = self.parse_input(true);
        for (i, s) in data.iter().enumerate() {
            sum += (i as u64 + 1) * s.bid as u64;
        }
        sum
    }
}


#[cfg(test)]
mod tests {
    use crate::year2023::day7::Hand;

    #[test]
    fn test_largest_key() {
        let hand = "HHDDUUUUI";
        let freq_map = Hand::get_frequencies(hand);
        let largest_key = Hand::largest_key(&freq_map).unwrap_or(&' ');
        assert_eq!(*largest_key, 'U');
    }
}

