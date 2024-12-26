use ahash::{HashMap, HashSet};
use rand::{rngs::ThreadRng};
use std::fs;

use crate::traits::Day;

const AND: u8 = 0;
const OR: u8 = 1;
const XOR: u8 = 2;

const MAX_Z: u64 = 45;

//type alias for graph
type Graph = HashMap<String, (String, u8, String)>;

pub struct Day24 {
    inputs: HashMap<String, bool>,
    dag: Graph,
}

impl Day24 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2024/day24").unwrap();
        let mut parts = data.split("\n\n");
        let mut inputs = HashMap::default();
        let mut dag = HashMap::default();

        for line in parts.next().unwrap().lines() {
            let mut section = line.split(": ");
            let wire = section.next().unwrap();
            let value = section.next().unwrap();
            let value = if value == "1" { true } else { false };
            inputs.insert(wire.to_string(), value);
        }

        for line in parts.next().unwrap().lines() {
            let mut section = line.split(" -> ");
            let mut input_parts = section.next().unwrap().split(" ");
            let a = input_parts.next().unwrap();
            let op = match input_parts.next() {
                Some("AND") => AND,
                Some("OR") => OR,
                Some("XOR") => XOR,
                _ => panic!("Invalid operation"),
            };
            let b = input_parts.next().unwrap();

            let output = section.next().unwrap();
            dag.insert(output.to_string(), (a.to_string(), op, b.to_string()));
        }

        Day24 { inputs, dag }
    }
}

#[allow(dead_code)]
fn dfs(
    graph: &Graph,
    node: &str,
    visited: &mut HashSet<String>,
    mut end_node: Option<&mut Vec<String>>,
) {
    if node.starts_with('x') || node.starts_with('y') {
        if let Some(end_node) = end_node {
            end_node.push(node.to_string());
        }
        return;
    }
    if visited.contains(node) {
        return;
    }
    visited.insert(String::from(node));
    if let Some((a, _, b)) = graph.get(node) {
        dfs(graph, a, visited, end_node.as_deref_mut());
        dfs(graph, b, visited, end_node.as_deref_mut());
    }
}

fn traverse_graph(
    graph: &Graph,
    node: &str,
    inputs: &HashMap<String, bool>,
    cache: &mut HashMap<String, bool>,
    visited: &mut HashSet<String>,
) -> Result<bool, String> {
    if let Some(val) = cache.get(node) {
        return Ok(*val);
    }
    if let Some(val) = inputs.get(node) {
        return Ok(*val);
    }
    if visited.contains(node) {
        return Err("Loop found".to_string());
    }
    visited.insert(node.to_string());

    if let Some((a, op, b)) = graph.get(node) {
        let a_val = traverse_graph(graph, a, inputs, cache, visited)?;
        let b_val = traverse_graph(graph, b, inputs, cache, visited)?;
        let result = match *op {
            AND => a_val & b_val,
            OR => a_val | b_val,
            XOR => a_val ^ b_val,
            _ => panic!("Invalid operation"),
        };
        cache.insert(node.to_string(), result);
        Ok(result)
    } else {
        Err("Loop found".to_string())
    }
}

fn complete_circuit(graph: &Graph, inputs: &HashMap<String, bool>) -> Result<Vec<bool>, String> {
    let mut result = vec![];
    let mut cache = HashMap::default();
    for i in 0..=MAX_Z {
        let z = format!("z{:02}", i);
        let mut visited = HashSet::default();
        let val = traverse_graph(graph, &z, inputs, &mut cache, &mut visited)?;
        result.push(val);
    }
    Ok(result)
}

#[allow(dead_code)]
fn print_tree(tree: &Graph, root: &str, indent: usize) {
    println!("{}{}", " ".repeat(indent * 4), root);

    if let Some((left, _, right)) = tree.get(root) {
        print_tree(tree, left, indent + 1);
        print_tree(tree, right, indent + 1);
    }
}

fn get_mismatch_idx(result: &Vec<bool>, actual: &Vec<bool>) -> Vec<usize> {
    let mut mismatch_idx = vec![];
    for (idx, (a, b)) in result.iter().zip(actual.iter()).enumerate() {
        if a != b {
            mismatch_idx.push(idx);
        }
    }
    mismatch_idx
}

fn reduce(
    all_nodes: &Vec<String>,
    dag: &mut Graph,
    inputs: &HashMap<String, bool>,
    result: &Vec<bool>,
    original_mismatch: usize,
    depth: usize,
    swaps: &mut Vec<(usize, usize)>,
    all_swaps: &mut Vec<(String, String)>,
) {
    if depth == 4 {
        return;
    }

    for i in 0..all_nodes.len() - 1 {
        let mut found = false;
        for (a, b) in swaps.iter() {
            if i == *a || i == *b {
                found = true;
                break;
            }
        }
        if found {
            continue;
        }

        for j in i + 1..all_nodes.len() {
            let mut found = false;
            for (a, b) in swaps.iter() {
                if j == *a || j == *b {
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }

            let node_i = &all_nodes[i];
            let node_j = &all_nodes[j];

            // swap ith and jth node
            let temp = dag.remove(node_i).unwrap();
            let temp2 = dag.remove(node_j).unwrap();
            dag.insert(node_i.to_string(), temp2);
            dag.insert(node_j.to_string(), temp);
            swaps.push((i, j));
            match complete_circuit(&dag, &inputs) {
                Ok(actual) => {
                    let mismatch_idx = get_mismatch_idx(&result, &actual);
                    if mismatch_idx.len() == 0 {
                        for (a, b) in swaps.iter() {
                            all_swaps.push((all_nodes[*a].to_string(), all_nodes[*b].to_string()));
                        }
                    } else if mismatch_idx.len() < original_mismatch {
                        reduce(
                            &all_nodes,
                            dag,
                            &inputs,
                            result,
                            mismatch_idx.len(),
                            depth + 1,
                            swaps,
                            all_swaps,
                        );
                    }
                }
                Err(_) => {}
            }
            swaps.pop();
            let temp = dag.remove(node_i).unwrap();
            let temp2 = dag.remove(node_j).unwrap();
            dag.insert(node_i.to_string(), temp2);
            dag.insert(node_j.to_string(), temp);
        }
    }
}

fn num_to_padded_binary(num: u64, padding: u64) -> Vec<bool> {
    let mut result = vec![];
    let mut num = num;
    while num > 0 {
        result.push(num & 1 == 1);
        num >>= 1;
    }
    for _ in 0..(padding + 1 - result.len() as u64) {
        result.push(false);
    }

    result
}

fn bin_vec_to_num(bin: &Vec<bool>) -> u64 {
    bin.iter()
        .rev()
        .fold(0, |acc, val| acc << 1 | (*val as u64))
}

#[allow(dead_code)]
fn print_bool_vec(bin: &Vec<bool>, rev: bool) {
    let iter: Box<dyn Iterator<Item = &bool>> = if rev {
        Box::new(bin.iter().rev())
    } else {
        Box::new(bin.iter())
    };

    for b in iter {
        print!("{}", if *b { 1 } else { 0 });
    }
    println!();
}

fn produce_input_map(x: u64, y: u64) -> HashMap<String, bool> {
    let x_bool = num_to_padded_binary(x, MAX_Z - 1);
    let y_bool = num_to_padded_binary(y, MAX_Z - 1);
    let mut inputs = HashMap::default();
    for i in 0..MAX_Z {
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        inputs.insert(x, x_bool[i as usize]);
        inputs.insert(y, y_bool[i as usize]);
    }
    inputs
}

fn get_xy(bits: u8, _rng: &mut ThreadRng) -> (u64, u64) {
    if bits == 0 {
        return (0, 0);
    }
    // let high = 1u64 << bits;
    // let low = 1u64 << (bits - 1);
    // let n: u64 = rng.gen_range(low..high);
    
    let mut n = 1u64 << (bits-1);
    n -= 1;

    (n, 3)
}

#[allow(dead_code)]
fn get_node_children(graph: &Graph, node: &str) -> HashSet<String> {
    let mut visited = HashSet::default();
    dfs(&graph, node, &mut visited, None);
    visited
}

#[allow(dead_code)]
fn print_graph_nodes(graph: &Graph) {
    for i in 0..MAX_Z {
        let z = format!("z{:02}", i);
        let mut visited = HashSet::default();
        let mut children = vec![];
        dfs(&graph, &z, &mut visited, Some(&mut children));
        let ch: Vec<u64> = children
            .iter()
            .filter_map(|x| {
                if x.starts_with('x') {
                    Some(x[1..].parse().unwrap())
                } else {
                    None
                }
            })
            .collect();
        println!("{z} = {:?}", ch);
    }
}

fn get_fail_point(dag: &Graph, rng: &mut ThreadRng, start: u8) -> Option<(u8, u64, u64)> {
    let mut fail_point: Option<(u8, u64, u64)> = None;

    for i in start..46 {
        let (x, y) = get_xy(i, rng);
        let input_map = produce_input_map(x, y);
        match complete_circuit(&dag, &input_map) {
            Ok(result) => {
                let z = bin_vec_to_num(&result);
                if z != x + y {
                    fail_point = Some((i, x, y));
                    break;
                }
            }
            Err(_) => {
                fail_point = Some((i, x, y));
                break;
            }
        }
    }
    fail_point
}

fn one_iter(dag: &mut Graph, depth: u8) -> Result<Vec<(String, String)>, String> {
    let mut rng = rand::thread_rng();
    let fail_point = get_fail_point(&dag, &mut rng, 0);
    if depth == 4 {
        if fail_point.is_none() {
            return Ok(vec![]);
        } else {
            return Err("Failed".to_string());
        }
    }

    if fail_point.is_none() {
        return Ok(vec![]);
    }
    let (i, x, y) = fail_point.unwrap();
    let input_map = produce_input_map(x, y);
    let result = complete_circuit(&dag, &input_map)?;

    let expected = num_to_padded_binary(x + y, MAX_Z);
    let mismatch_idx = get_mismatch_idx(&result, &expected);

    let child_5 = get_node_children(&dag, &format!("z{:02}", i - 3));
    let child_6 = get_node_children(&dag, &format!("z{:02}", i - 2));
    let child_7 = get_node_children(&dag, &format!("z{:02}", i - 1));
    let child_8 = get_node_children(&dag, &format!("z{:02}", i));
    let diff87 = child_8
        .difference(&child_7)
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();
    let diff76 = child_7
        .difference(&child_6)
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();

    let diff65 = child_6
        .difference(&child_5)
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();

    let u1 = diff87
        .union(&diff76)
        .map(String::from)
        .collect::<HashSet<_>>();
    let u2 = diff76
        .union(&diff65)
        .map(String::from)
        .collect::<HashSet<_>>();
    let nodes = u1.union(&u2).map(String::from).collect::<Vec<_>>();

    let mut swaps = vec![];
    let mut all_swaps = vec![];
    reduce(
        &nodes,
        dag,
        &input_map,
        &expected,
        mismatch_idx.len(),
        3,
        &mut swaps,
        &mut all_swaps,
    );
    Ok(all_swaps)
}

fn do_swap(dag: &mut Graph, swap: (String, String)) {
    let temp1 = dag.remove(&swap.0).unwrap();
    let temp2 = dag.remove(&swap.1).unwrap();
    dag.insert(swap.0.to_string(), temp2);
    dag.insert(swap.1.to_string(), temp1);
}

fn solution_to_string(solution: &Vec<(String, String)>) -> String {
    let mut result = vec![];
    for s in solution {
        result.push(s.0.to_string());
        result.push(s.1.to_string());
    }
    result.sort();

    result.join(",")
}

impl Day for Day24 {
    fn part_1(&mut self) -> u64 {
        let result = complete_circuit(&self.dag, &self.inputs).expect("No solution found");
        bin_vec_to_num(&result)
    }

    fn part_2(&mut self) -> u64 {
        let mut all_solutions = vec![];
        let swaps1 = one_iter(&mut self.dag, 0).expect("First iteration failed");
        for swap in swaps1.iter() {
            do_swap(&mut self.dag, swap.clone());
            let swaps2 = one_iter(&mut self.dag, 1);
            if swaps2.is_err() {
                do_swap(&mut self.dag, swap.clone());
                continue;
            }
            let swaps2 = swaps2.unwrap();
            for swap2 in swaps2.iter() {
                do_swap(&mut self.dag, swap2.clone());
                let swaps3 = one_iter(&mut self.dag, 2);
                if swaps3.is_err() {
                    do_swap(&mut self.dag, swap2.clone());
                    continue;
                }
                let swaps3 = swaps3.unwrap();
                for swap3 in swaps3.iter() {
                    do_swap(&mut self.dag, swap3.clone());
                    let swaps4 = one_iter(&mut self.dag, 3);
                    if swaps4.is_err() {
                        do_swap(&mut self.dag, swap3.clone());
                        continue;
                    }
                    let swaps4 = swaps4.unwrap();
                    for swap4 in swaps4.iter() {
                        do_swap(&mut self.dag, swap4.clone());
                        let res = one_iter(&mut self.dag, 4);
                        if res.is_err() {
                            do_swap(&mut self.dag, swap4.clone());
                            continue;
                        }
                        let sol = vec![swap.clone(), swap2.clone(), swap3.clone(), swap4.clone()];
                        all_solutions.push(sol);

                        do_swap(&mut self.dag, swap4.clone());
                    }
                    do_swap(&mut self.dag, swap3.clone());
                }
                do_swap(&mut self.dag, swap2.clone());
            }
            do_swap(&mut self.dag, swap.clone());
        }

        let mut rng = rand::thread_rng();
        let mut visited = HashSet::default();
        for solution in all_solutions {
            let s_str = solution_to_string(&solution);
            if visited.contains(&s_str) {
                continue;
            }
            for sol in solution.iter() {
                do_swap(&mut self.dag, sol.clone());
            }
            visited.insert(s_str);

            let mut found = true;
            for _ in 0..2 {
                let fail_point = get_fail_point(&self.dag, &mut rng, 2);
                if fail_point.is_some() {
                    found = false;
                    break;
                }
            }
            if found {
                println!(
                    "Found solution: {:?} ;; {}",
                    solution,
                    solution_to_string(&solution)
                );
            }

            for sol in solution.iter().rev() {
                do_swap(&mut self.dag, sol.clone());
            }
        }
        // regression_test_input(&mut self.dag);
        0
    }
}

#[allow(dead_code)]
fn regression_test_input(dag: &mut Graph) {
    let all_solutions = vec![
        [("shj", "z07"), ("wkb", "tpk"), ("z23", "pfn"), ("z27", "z28")],
        [("shj", "z07"), ("wkb", "tpk"), ("z23", "pfn"), ("z27", "kcd")],
        [("shj", "z07"), ("wkb", "tpk"), ("z23", "pfn"), ("z27", "wvb")],
    ];

    let mut visited = HashSet::default();
    let mut rng = rand::thread_rng();
    for s in all_solutions {
        let solution = s
            .to_vec()
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        let s_str = solution_to_string(&solution);
        if visited.contains(&s_str) {
            continue;
        }
        visited.insert(s_str);
        for sol in solution.iter() {
            do_swap(dag, sol.clone());
        }

        for _ in 0..1 {
            let fail_point = get_fail_point(&dag, &mut rng, 40);
            if fail_point.is_some() {
                println!("Bad: {:?} ;; {}", solution, solution_to_string(&solution));
                break;
            }
        }

        for sol in solution.iter().rev() {
            do_swap(dag, sol.clone());
        }
    }
}
