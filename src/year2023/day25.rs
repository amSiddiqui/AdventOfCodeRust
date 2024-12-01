use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::traits::Day;

pub struct Day25 {
    graph: HashMap<u64, Vec<u64>>,
    node_map: HashMap<u64, String>,
}

impl Day25 {
    pub fn new() -> Self {
        let mut graph = HashMap::new();
        let mut node_map = HashMap::new();

        // Use this for testing with the sample input
//         let input = "\
// jqt: rhn xhk nvd
// rsh: frs pzl lsr
// xhk: hfx
// cmg: qnr nvd lhk bvb
// rhn: xhk bvb hfx
// bvb: xhk hfx
// pzl: lsr hfx nvd
// qnr: nvd
// ntq: jqt hfx bvb xhk
// nvd: lhk
// lsr: lhk
// rzs: qnr cmg lsr rsh
// frs: qnr lhk lsr";

        //input
        fs::read_to_string("data/year2023/day25")
            .expect("Cannot read data file")
            .split('\n')
            .for_each(|line| {
                let parts = line.split(':').collect::<Vec<&str>>();
                let key = hash_string(parts[0].trim(), &mut node_map);
                let vals = parts[1]
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| hash_string(s, &mut node_map))
                    .collect::<Vec<u64>>();
                graph.insert(key, vals.clone());
                for &child in &vals {
                    graph.entry(child).or_insert_with(Vec::new).push(key);
                }
            });
        
        Day25 { graph, node_map }
    }

    fn connected_components(&self, graph: &HashMap<u64, Vec<u64>>) -> Vec<HashSet<u64>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for &node in graph.keys() {
            if !visited.contains(&node) {
                let mut component = HashSet::new();
                self.dfs_collect(node, graph, &mut visited, &mut component);
                components.push(component);
            }
        }
        components
    }

    fn dfs_collect(
        &self,
        node: u64,
        graph: &HashMap<u64, Vec<u64>>,
        visited: &mut HashSet<u64>,
        component: &mut HashSet<u64>,
    ) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        component.insert(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                self.dfs_collect(neighbor, graph, visited, component);
            }
        }
    }
}

impl Day for Day25 {
    fn part_1(&mut self) -> u64 {
        let mut edge_set = HashSet::new();
        for (&node, neighbors) in &self.graph {
            for &neighbor in neighbors {
                let (a, b) = if node < neighbor {
                    (node, neighbor)
                } else {
                    (neighbor, node)
                };
                edge_set.insert((a, b));
            }
        }
        let edges: Vec<(u64, u64)> = edge_set.into_iter().collect();

        let combinations = edges.iter().combinations(3);

        let result = combinations
            .into_iter()
            .map(|edge_combo| {
                let mut test_graph = self.graph.clone();
                for &&(u, v) in &edge_combo {
                    if let Some(neighbors) = test_graph.get_mut(&u) {
                        neighbors.retain(|&n| n != v);
                    }
                    if let Some(neighbors) = test_graph.get_mut(&v) {
                        neighbors.retain(|&n| n != u);
                    }
                }
                let components = self.connected_components(&test_graph);
                if components.len() == 2 {
                    let sizes = components.iter().map(|c| c.len()).collect::<Vec<_>>();
                    let product = sizes[0] * sizes[1];
                    Some((product, edge_combo.clone()))
                } else {
                    None
                }
            })
            .filter_map(|x| x)
            .max_by_key(|&(product, _)| product);

        if let Some((max_product, best_combo)) = result {
            println!("Maximum product: {}", max_product);
            println!("Edges to disconnect:");
            for &(u, v) in best_combo {
                println!(
                    "{} - {}",
                    self.node_map.get(&u).unwrap(),
                    self.node_map.get(&v).unwrap()
                );
            }
            max_product as u64
        } else {
            0
        }
    }

    fn part_2(&mut self) -> u64 {
        0
    }
}

fn hash_string(s: &str, map: &mut HashMap<u64, String>) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let res = hasher.finish();
    map.insert(res, s.to_string());
    res
}
