use ahash::{AHashMap, AHashSet};

use crate::traits::Day;
use std::fs;

pub struct Day11 {
    graph: AHashMap<String, Vec<String>>,
}
impl Day11 {
    pub fn new() -> Self {
        let input = fs::read_to_string("data/year2025/day11").expect("Cannot read data");
        let data = input.lines().map(|line| {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let vals = parts.next().unwrap();
            let vals: Vec<String> = vals.split(' ').map(String::from).collect();
            (String::from(key), vals)
        });
        let map = AHashMap::from_iter(data);
        Day11 { graph: map }
    }
}

fn traverse(graph: &AHashMap<String, Vec<String>>, current_node: &str, paths: u64) -> u64 {
    if current_node == "out" {
        return 1;
    }
    if !graph.contains_key(current_node) {
        return 0;
    }

    let mut new_paths = 0;
    for child in graph.get(current_node).unwrap() {
        new_paths += traverse(graph, &child, paths);
    }

    paths + new_paths
}

fn traverse_2(
    graph: &AHashMap<String, Vec<String>>,
    current_node: &str,
    paths: u64,
    target: &str,
    visited: &mut AHashSet<String>,
    to_avoid: &AHashSet<String>
) -> u64 {
    if to_avoid.contains(current_node) {
        return 0;
    }
    if current_node == target {
        return 1;
    }
    visited.insert(current_node.to_string());
    let mut new_paths = 0;
    for child in graph.get(current_node).unwrap() {   
        let res = traverse_2(graph, &child, paths, target, visited, to_avoid);
        new_paths += res;
        
    }
    paths + new_paths
}

impl Day for Day11 {
    fn part_1(&mut self) -> u64 {
        traverse(&self.graph, "you", 0)
    }
    fn part_2(&mut self) -> u64 {
        let mut visited = AHashSet::default();
        let mut to_avoid = AHashSet::default();
        let dac_p = traverse_2(
            &self.graph,
            "dac",
            0,
            "out",
            &mut visited,
            &to_avoid
        );
        to_avoid.extend(visited);
        visited = AHashSet::default();
        println!("dac_p : {}, visited: {}, to_avoid: {}", dac_p, visited.len(), to_avoid.len());
        to_avoid.remove("dac");
        let fft_p = traverse_2(
            &self.graph,
            "fft",
            0,
            "dac",
            &mut visited,
            &to_avoid
        );
        to_avoid.extend(visited);
        visited = AHashSet::default();
        to_avoid.remove("dac");
        to_avoid.remove("fft");
        println!("fft_p : {}, visited: {}, to_avoid: {}", fft_p, visited.len(), to_avoid.len());
        let res = traverse_2(
            &self.graph,
            "svr",
            0,
            "fft",
            &mut visited,
            &to_avoid
        );

        res * fft_p * dac_p
        
    }
}
