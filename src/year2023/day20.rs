use std::collections::{HashMap, VecDeque};
use std::fs;
use crate::traits::Day;
use crate::year2023::day8::Day8;

pub struct Day20 {
    mapping: HashMap<String, Vec<String>>,
    states: HashMap<String, State>
}


#[derive(Debug)]
pub struct State {
    ff: bool,
    con: Option<HashMap<String, bool>>
}

impl Day20 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day20")
            .expect("Cannot read file")
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>();

        let mut mapping: HashMap<String, Vec<String>> = HashMap::new();
        let mut states: HashMap<String, State> = HashMap::new();
        for line in lines {
            let (mut key, val) = line.split_once(" -> ").expect("Line malformed");
            let vals = val.split(", ");
            let first_char = key.as_bytes()[0] as char;
            if first_char == '%' {
                key = &key[1..];
                let state = State {
                    ff: false,
                    con: None
                };
                states.insert(String::from(key), state);
            } else if first_char == '&' {
                key = &key[1..];
                let state = State {
                    ff: false,
                    con: Some(HashMap::new())
                };
                states.insert(String::from(key), state);
            }

            mapping.insert(String::from(key), vals.map(String::from).collect::<Vec<_>>());
        }

        states.iter_mut().for_each(|(k, v)| {
            if let Some(km) = &mut v.con {
                for (kmap, vmap) in &mapping {
                    if vmap.contains(k) {
                        km.insert(kmap.clone(), false);
                    }
                }
            }
        });

        Day20 {
            mapping,
            states
        }
    }

    fn reset_state(&mut self) {
        self.states.iter_mut().for_each(|(_, v)| {
            v.ff = false;
            if let Some(con) = &mut v.con {
                con.values_mut().for_each(|x| {
                    *x = false;
                });
            }
        });
    }

    fn get_neg_pos_count(&mut self) -> (i32, i32) {
        let mut queue:VecDeque<(String, bool, String)> = VecDeque::new();
        let low_sigs = self.mapping.get("broadcaster").unwrap();
        for sig in low_sigs {
            queue.push_back((String::from("broadcaster"), false, sig.clone()));
        }
        let mut neg = 0;
        let mut pos = 0;
        while let Some((f_node, pulse, node)) = queue.pop_front() {
            if pulse {
                pos += 1;
            } else {
                neg += 1;
            }
            self.update_states(&mut queue, f_node, pulse, &node);
        }
        (pos, neg + 1)
    }

    fn find_node(&mut self, needle: &str) -> bool {
        let mut queue:VecDeque<(String, bool, String)> = VecDeque::new();
        let low_sigs = self.mapping.get("broadcaster").unwrap();
        for sig in low_sigs {
            queue.push_back((String::from("broadcaster"), false, sig.clone()));
        }
        while let Some((f_node, pulse, node)) = queue.pop_front() {
            if f_node == needle && pulse {
                return true;
            }
            self.update_states(&mut queue, f_node, pulse, &node);
        }

        false
    }

    fn update_states(&mut self, queue: &mut VecDeque<(String, bool, String)>, f_node: String, pulse: bool, node: &String) {
        if let Some(state) = self.states.get_mut(node) {
            if let Some(con) = &mut state.con {
                con.insert(f_node, pulse);
                let new_pulse = !con.values().all(|x| *x);
                for i in self.mapping.get(node).unwrap() {
                    queue.push_back((node.clone(), new_pulse, i.clone()));
                }
            } else if !pulse {
                let new_pulse = !state.ff;
                state.ff = new_pulse;
                for i in self.mapping.get(node).unwrap() {
                    queue.push_back((node.clone(), new_pulse, i.clone()));
                }
            }
        }
    }
}

impl Day for Day20 {
    fn part_1(&mut self) -> u64 {
        self.reset_state();
        let mut p = 0_u64;
        let mut n = 0_u64;
        for _ in 0..1000 {
            let (a, b) = self.get_neg_pos_count();
            p += a as u64;
            n += b as u64;
        }
        p * n
    }

    fn part_2(&mut self) -> u64 {
        let nodes = ["tx", "dd", "nz", "ph"];
        let mut vals: Vec<i32> = Vec::new();

        for node in nodes {
            self.reset_state();
            let mut count = 1;
            while !self.find_node(node) {
                count += 1;
            }
            vals.push(count);
        }

        Day8::lcm(vals)
    }
}

