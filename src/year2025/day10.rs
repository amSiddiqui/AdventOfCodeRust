use ahash::{AHashMap, AHashSet};
use rayon::prelude::*;

use crate::traits::Day;
use std::{fs, u16};

pub struct Day10 {
    lines: Vec<String>,
}

impl Day10 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day10")
            .expect("Cannot read data")
            .lines()
            .map(|line| String::from(line))
            .collect();
        Day10 { lines: data }
    }
}

fn parse_diagram_to_config(d: &str) -> AHashSet<u16> {
    let mut res = AHashSet::new();
    let char_arr: Vec<char> = d.chars().collect();
    for (i, c) in (&char_arr[1..char_arr.len() - 1]).iter().enumerate() {
        if *c == '#' {
            res.insert(i as u16);
        }
    }
    res
}

fn tree_search(
    config: &AHashSet<u16>,
    buttons: &Vec<AHashSet<u16>>,
    current_state: &AHashSet<u16>,
    iter: i32,
    smallest_iter: i32,
    cache: &mut AHashMap<(i32, Vec<u16>), i32>,
) -> i32 {
    // println!("{current_state:?}  {iter}");
    let mut cur_vec: Vec<u16> = current_state.iter().cloned().collect();
    cur_vec.sort_unstable();
    // if cur_vec.len() > 0 && fails.contains(&cur_vec) {
    //     return -1;
    // }
    if let Some(res) = cache.get(&(iter, cur_vec.clone())) {
        return *res;
    }
    if iter == 10 {
        // fails.insert(cur_vec);
        cache.insert((iter, cur_vec), -1);
        return -1;
    }
    if smallest_iter != -1 && iter > smallest_iter {
        //fails.insert(cur_vec);
        cache.insert((iter, cur_vec), -1);
        return -1;
    }
    if config == current_state {
        cache.insert((iter, cur_vec), iter);
        return iter;
    }
    // let need = &config
    //     .symmetric_difference(&current_state)
    //     .copied()
    //     .collect();

    // let closeness = buttons
    //     .iter()
    //     .map(|b| b.intersection(need).count())
    //     .collect::<Vec<usize>>();

    // let largest = closeness.iter().max().expect("Max to exists");
    // let candidates = closeness
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(i, val)| if val == largest { Some(i) } else { None })
    //     .collect::<Vec<usize>>();

    // apply candidates
    let mut results = vec![];
    let mut s = smallest_iter;
    for button_press in buttons {
        // let button_press = &buttons[candidate];
        let mut new_current_state = current_state.clone();
        for b in button_press {
            if new_current_state.contains(b) {
                new_current_state.remove(b);
            } else {
                new_current_state.insert(*b);
            }
        }
        let res = tree_search(config, buttons, &new_current_state, iter + 1, s, cache);
        if res != -1 {
            results.push(res);
            if res < s {
                s = res;
            }
        }
    }
    if results.len() == 0 {
        cache.insert((iter, cur_vec), -1);
        return -1;
    } else {
        let res = *results.iter().min().expect("No results found");
        cache.insert((iter, cur_vec), res);
        return res;
    }
}

#[allow(dead_code)]
fn tree_search_3(config: &mut Vec<u16>, buttons: &mut Vec<(Vec<u32>, bool)>, depth: u64, smallest: &mut u64) {
    // smallest index in config
    assert!(config.len() > 0, "Config is empty");
    if depth >= *smallest {
        return;
    }

    let min_val = config.iter().min().unwrap();
    let val = *min_val;
    if val == u16::MAX {
        if depth < *smallest {
            *smallest = depth;
        }
        // println!("==Found!!== :: {depth}");
        return;
    }
    let smallest_idxs = config
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if *v == val {Some(i)} else { None })
        .collect::<Vec<_>>();


    let buttons_to_press = buttons
        .iter()
        .enumerate()
        .filter(|(_, (button, used))| {
            if *used {
                return false;
            }
            for &b in button {
                if config[b as usize] == u16::MAX {
                    return false;
                }
            }

            true
        })
        .filter_map(|(i, (button, _))| {
            for s in smallest_idxs.iter() {
                if button.contains(&(*s as u32)) {
                    return Some(i);
                }
            }
            return None;
        })
        .collect::<Vec<_>>();

    

    // print!("[");
    // config.iter()
    // .for_each(|&c| {
    //     if c == u16::MAX {
    //         print!("  ∞, ")
    //     } else {
    //         print!("{c:3}, ");
    //     }
    // });
    // print!("] ;; ");
    
    // if buttons_to_press.len() > 0 {
    //     print!("{:?} ;; {:?} ;; ", buttons[buttons_to_press[0]].0, buttons_to_press.len());
    // }
    
    // buttons.iter()
    // .for_each(|(v, used)| {
    //     if !used {
    //         print!("\x1b[32m{:?}\x1b[0m, ", v);
    //     } else {
    //         print!("\x1b[31m{:?}\x1b[0m, ", v);
    //     }
        
    // });
    
    // println!();

    if buttons_to_press.len() == 0 {
        // println!("==Bust==");
        return;
    }

    if buttons_to_press.len() == 1 {
        for &button in buttons[buttons_to_press[0]].0.iter() {
            let b = button as usize;
            config[b] -= val;
            if config[b] == 0 {
                config[b] = u16::MAX;
            }
        }
        
        buttons[buttons_to_press[0]].1 = true;

        tree_search_3(config, buttons, depth+(val as u64), smallest);
        buttons[buttons_to_press[0]].1 = false;
        for &button in buttons[buttons_to_press[0]].0.iter() {
            let b = button as usize;
            if config[b] == u16::MAX {
                config[b] = 0;
            }
            config[b] += val;
        }
    } else {
        for v in (0..=val).rev() {
            for &button in buttons[buttons_to_press[0]].0.iter() {
                let b = button as usize;
                config[b] -= v;
                if config[b] == 0 {
                    config[b] = u16::MAX;
                }
            }
            buttons[buttons_to_press[0]].1 = true;
            tree_search_3(config, buttons, depth+(v as u64), smallest);
            buttons[buttons_to_press[0]].1 = false;
            for &button in buttons[buttons_to_press[0]].0.iter() {
                let b = button as usize;
                if config[b] == u16::MAX {
                    config[b] = 0;
                }
                config[b] += v;
            }
        }
    }
}

fn parse_joltage(input_str: &str) -> Vec<u16> {
    let mut result = vec![];
    let input = &input_str[1..input_str.len() - 1];
    for p in input.split(',') {
        result.push(u16::from_str_radix(p, 10).expect("Cannot parse"))
    }
    result
}

impl Day for Day10 {
    fn part_1(&mut self) -> u64 {
        let data: Vec<(AHashSet<u16>, Vec<AHashSet<u16>>)> = self
            .lines
            .iter()
            .map(|line| {
                let mut parts = line.split(' ');
                let mut buttons = vec![];
                let first = parts.next().expect("first part not found");
                let final_config = parse_diagram_to_config(first);
                for button in parts.rev().skip(1) {
                    let button = &button[1..button.len() - 1];
                    let mut b = AHashSet::new();
                    for digit in button.split(',') {
                        b.insert(u16::from_str_radix(digit, 10).expect("Cannot parse"));
                    }
                    buttons.push(b);
                }
                buttons.reverse();
                (final_config, buttons)
            })
            .collect();

        let solution = data
            .par_iter()
            .map(|(config, buttons)| {
                let current_state: AHashSet<u16> = AHashSet::default();
                let mut cache = AHashMap::default();
                let res = tree_search(&config, &buttons, &current_state, 0, -1, &mut cache);
                if res == -1 {
                    panic!("Cannot find solution for {config:?}");
                }
                // println!("Config: {config:?};; Result = {res}");
                // print!("Buttons: ");
                // for button in buttons {
                //     print!("{button:?}, ");
                // }
                // println!("\n");
                res as u64
            })
            .sum();
        solution
    }
    fn part_2(&mut self) -> u64 {
        let mut data: Vec<(Vec<u16>, Vec<Vec<u32>>)> = self
            .lines
            .iter()
            .map(|line| {
                let mut parts = line.split(' ').rev();
                let mut buttons = vec![];
                let first = parts.next().expect("first part not found");
                let final_config = parse_joltage(first);
                for button in parts.rev().skip(1) {
                    let button = &button[1..button.len() - 1];
                    let mut b = vec![];
                    for digit in button.split(',') {
                        b.push(u32::from_str_radix(digit, 10).expect("Cannot parse"));
                    }
                    buttons.push(b);
                }
                // buttons.reverse();
                buttons.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
                (final_config, buttons)
            })
            .collect();
        

        let solution = data
            .par_iter_mut()
            .map(|(config, buttons_plain)| {
                let mut buttons = buttons_plain
                .into_iter()
                .map(|v| (v.clone(), false))
                .collect();
                let mut res = u64::MAX;
                tree_search_3(config, &mut buttons, 0, &mut res);
                if res == u64::MAX {
                    panic!("Result not found");
                }
                println!("Config: {config:?};; Result = {res}");
                // print!("Buttons: ");
                // for button in buttons {
                //     print!("{button:?}, ");
                // }
                // println!("\n");

                res
            })
            .sum();
        // let solution = 0;
        solution
    }
}
