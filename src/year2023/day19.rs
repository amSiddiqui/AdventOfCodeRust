use std::collections::HashMap;
use std::fs;
use crate::traits::Day;

pub struct Day19 {
    workflows: HashMap<String, Vec<String>>,
    parts: Vec<HashMap<char, u32>>,
}

impl Day19 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/day19")
            .expect("Cannot read input file");
        let (workflow_str, parts_str) = data
            .split_once("\n\n")
            .expect("Input not present");

        let workflows: HashMap<String, Vec<String>> = workflow_str.split('\n')
            .map(|line| {
                let (key, mut vec_str) = line.split_once('{').expect("Line malformed");
                vec_str = &vec_str[..vec_str.len() - 1];
                (String::from(key), vec_str.split(',').map(String::from).collect::<Vec<_>>())
            }).collect();

        let parts = parts_str.split('\n')
            .map(|mut line| {
                line = &line[1..line.len() - 1];
                line.split(',').map(|kv| {
                    let (k, v) = kv.split_once('=').expect("Mapping incorrect");
                    (k.as_bytes()[0] as char, v.parse::<u32>().expect("Parts value in correct"))
                }).collect()
            }).collect::<Vec<_>>();
        Day19 {
            workflows,
            parts,
        }
    }

    fn is_part_accepted(&self, part: &HashMap<char, u32>) -> bool {
        let mut current_condition = "in";
        loop {
            if current_condition == "A" {
                return true;
            }
            if current_condition == "R" {
                return false;
            }
            let conditions = self.workflows.get(current_condition).expect("Condition not found");
            let mut found = false;
            for condition in conditions {
                if condition == "A" {
                    return true;
                }
                if condition == "R" {
                    return false;
                }
                if let Some((cond, nxt)) = condition.split_once(':') {
                    let a = cond.as_bytes()[0] as char;
                    let op = cond.as_bytes()[1] as char;
                    let val = cond[2..].parse::<u32>().expect("Condition does not have valid number");
                    if op == '>' {
                        if part.get(&a).unwrap_or_else(|| panic!("Unknown key {a}")) > &val {
                            current_condition = nxt;
                            found = true;
                            break;
                        }
                    } else if op == '<' && part.get(&a).unwrap_or_else(|| panic!("Unknown key {a}")) < &val {
                        current_condition = nxt;
                        found = true;
                        break;
                    }
                } else {
                    current_condition = condition;
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
    }

    fn get_range_combination(parts: &HashMap<char, [u32; 2]>) -> u64 {
        parts.values()
            .map(|res| res[1] as u64 - res[0] as u64 + 1)
            .product()
    }
}

impl Day for Day19 {
    fn part_1(&mut self) -> u64 {
        self.parts.iter()
            .filter_map(|part| if self.is_part_accepted(part) {
                Some(part.values().sum::<u32>() as u64)
            } else { None }).sum()
    }

    fn part_2(&mut self) -> u64 {
        let part_ranges: HashMap<char, [u32; 2]> = vec![
            ('x', [1, 4000]),
            ('m', [1, 4000]),
            ('a', [1, 4000]),
            ('s', [1, 4000])
        ].into_iter().collect();

        let mut stack = Vec::new();
        stack.push((part_ranges, "in"));
        let mut ttl = 0;
        while let Some((mut ranges, condition)) = stack.pop() {
            if condition == "A" {
                ttl += Day19::get_range_combination(&ranges);
                continue;
            } else if condition == "R" {
                continue;
            }

            let conditions = self.workflows.get(condition).unwrap_or_else(|| panic!("Unknown condition {condition}"));
            for condition in conditions {
                if condition == "A" {
                    ttl += Day19::get_range_combination(&ranges);
                    continue;
                } else if condition == "R" {
                    continue;
                }

                if let Some((cond, nxt)) = condition.split_once(':') {
                    let a = cond.as_bytes()[0] as char;
                    let op = cond.as_bytes()[1] as char;
                    let val = cond[2..].parse::<u32>().expect("Condition value not correct");
                    let vals = ranges.get(&a).unwrap();
                    let low = vals[0];
                    let high = vals[1];
                    let mut new_ranges = ranges.clone();
                    if op == '>' {
                        new_ranges.insert(a, [val+1, high]);
                        ranges.insert(a, [low, val]);
                    } else {
                        new_ranges.insert(a, [low, val-1]);
                        ranges.insert(a, [val, high]);
                    }
                    stack.push((new_ranges, nxt));
                } else {
                    stack.push((ranges, condition));
                    break;
                }
            }
        }

        ttl
    }
}

