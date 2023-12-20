use std::collections::{HashMap, VecDeque};
use std::fs;
use crate::traits::Day;

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

    fn process_state(&mut self) -> bool {
        let mut queue:VecDeque<(String, bool, String)> = VecDeque::new();
        let low_sigs = self.mapping.get("broadcaster").unwrap();
        for sig in low_sigs {
            queue.push_back((String::from("broadcaster"), false, sig.clone()));
        }
        let mut found = false;
        while let Some((f_node, pulse, node)) = queue.pop_front() {
            if &node == "rx" && !pulse {
                found = true;
                break;
            }

            if let Some(state) = self.states.get_mut(&node) {
                if let Some(con) = &mut state.con {
                    con.insert(f_node, pulse);
                    let new_pulse = !con.values().all(|x| *x);
                    for i in self.mapping.get(&node).unwrap() {
                        queue.push_back((node.clone(), new_pulse, i.clone()));
                    }
                } else if !pulse {
                    let new_pulse = !state.ff;
                    state.ff = new_pulse;
                    for i in self.mapping.get(&node).unwrap() {
                        queue.push_back((node.clone(), new_pulse, i.clone()));
                    }
                }
            }

        }
        found
    }
}

impl Day for Day20 {
    fn part_1(&mut self) -> u64 {
        // let mut p = 0_u64;
        // let mut n = 0_u64;
        // for _ in 0..1000 {
        //     let (a, b, _) = self.process_state();
        //     p += a as u64;
        //     n += b as u64;
        // }
        0
    }

    fn part_2(&mut self) -> u64 {
        let mut found = false;
        let mut count = 0u64;
        while !found {
            found = self.process_state();
            count += 1;
        }
        count
    }
}

