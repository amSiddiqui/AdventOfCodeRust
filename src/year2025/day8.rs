use ahash::AHashSet;

use crate::traits::Day;
use core::fmt;
use std::fs;
use std::collections::BinaryHeap;

#[derive(Hash, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64
}

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        let p1 = self.x - other.x;
        let p2 = self.y - other.y;
        let p3 = self.z - other.z;
        let d = p1 * p1 + p2 * p2 + p3 * p3;
        d
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Point { }

pub struct Day8 {
    data: Vec<Point>,
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    pair: (usize, usize),
    d: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.d.cmp(&other.d)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Day8 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day8")
            .expect("String not found")
            .lines()
            .map(|line| {
                let mut parts = line.split(",");
                let x = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
                let y = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
                let z = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
                Point { x, y , z }
            })
            .collect();
        Day8 { data }
    }
}

#[allow(dead_code)]
fn print_cluster(clusters: &Vec<AHashSet<Point>>) {
    for (i, c) in clusters.iter().enumerate() {
        println!("Cluster {i}");
        for p in c {
            println!("{p}");
        }
        println!();
    }
}


impl Day for Day8 {
    fn part_1(&mut self) -> u64 {
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        
        for i in 0..self.data.len()-1 {
            for j in i+1..self.data.len() {
                let d = self.data[i].distance(&self.data[j]);
                heap.push(Node { pair: (i, j), d });
            }
        }

        let mut clusters: Vec<AHashSet<Point>> = vec![];
        let mut count = 0;
        for node in heap.into_sorted_vec() {
            if count == 1000 {
                break;
            }
            
            let mut found = false;
            let mut merge_idx = vec![];
            for (i, cls) in clusters.iter_mut().enumerate() {
                if cls.contains(&self.data[node.pair.0]) && cls.contains(&self.data[node.pair.1]) {
                    found = true;
                    break;
                }
                if cls.contains(&self.data[node.pair.0]) || cls.contains(&self.data[node.pair.1]) {
                    merge_idx.push(i);
                }
            }
            if found {
                // println!("{}, {} ;; D = {}", self.data[node.pair.0], self.data[node.pair.1], node.d);
                // println!("Skip\n");
                count += 1;
                continue;
            }
            if merge_idx.len() == 0 {
                let mut cls = AHashSet::new();
                cls.insert(self.data[node.pair.0].clone());
                cls.insert(self.data[node.pair.1].clone());
                clusters.push(cls);
                count += 1;
            }
            else if merge_idx.len() == 1 {
                let cls = &mut clusters[merge_idx[0]];
                cls.insert(self.data[node.pair.0].clone());
                cls.insert(self.data[node.pair.1].clone());
                count += 1;
            } else {
                let mut cls = AHashSet::new();
                for &idx in &merge_idx {
                    cls.extend(clusters[idx].iter().cloned());
                }
                cls.insert(self.data[node.pair.0].clone());
                cls.insert(self.data[node.pair.1].clone());
                let mut new_cluster = vec![];
                new_cluster.push(cls);
                for (i, c) in clusters.into_iter().enumerate() {
                    if !merge_idx.contains(&i) {
                        new_cluster.push(c);
                    }
                }
                clusters = new_cluster;
                count += 1;
            }
            // println!("{}, {} ;; D = {}", self.data[node.pair.0], self.data[node.pair.1], node.d);
            // print_cluster(&clusters);
            // println!("\n\n");
        }
        clusters.sort_by(|a, b| b.len().cmp(&a.len()));
        println!("Total clusters: {}", clusters.len());
        let a = clusters[0].len() as u64;
        let b = clusters[1].len() as u64;
        let c = clusters[2].len() as u64;

        println!("{}, {}, {}", a, b, c);
        a * b * c
    }
    fn part_2(&mut self) -> u64 {
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        
        for i in 0..self.data.len()-1 {
            for j in i+1..self.data.len() {
                let d = self.data[i].distance(&self.data[j]);
                heap.push(Node { pair: (i, j), d });
            }
        }

        let mut clusters: Vec<AHashSet<Point>> = vec![];
        for point in &self.data {
            let mut st = AHashSet::new();
            st.insert(point.clone());
            clusters.push(st);
        }
        for node in heap.into_sorted_vec() {
            let mut found = false;
            let mut merge_idx = vec![];
            for (i, cls) in clusters.iter_mut().enumerate() {
                if cls.contains(&self.data[node.pair.0]) && cls.contains(&self.data[node.pair.1]) {
                    found = true;
                    break;
                }
                if cls.contains(&self.data[node.pair.0]) || cls.contains(&self.data[node.pair.1]) {
                    merge_idx.push(i);
                }
            }
            if found {
                // println!("{}, {} ;; D = {}", self.data[node.pair.0], self.data[node.pair.1], node.d);
                // println!("Skip\n");
                continue;
            }
            if merge_idx.len() == 0 {
                let mut cls = AHashSet::new();
                cls.insert(self.data[node.pair.0].clone());
                cls.insert(self.data[node.pair.1].clone());
                clusters.push(cls);
            }
            else if merge_idx.len() == 1 {
                let cls = &mut clusters[merge_idx[0]];
                cls.insert(self.data[node.pair.0].clone());
                cls.insert(self.data[node.pair.1].clone());
            } else {
                let mut cls = AHashSet::new();
                for &idx in &merge_idx {
                    cls.extend(clusters[idx].iter().cloned());
                }
                cls.insert(self.data[node.pair.0].clone());
                cls.insert(self.data[node.pair.1].clone());
                let mut new_cluster = vec![];
                new_cluster.push(cls);
                for (i, c) in clusters.into_iter().enumerate() {
                    if !merge_idx.contains(&i) {
                        new_cluster.push(c);
                    }
                }
                clusters = new_cluster;
            }
            // println!("{}, {} ;; D = {}", self.data[node.pair.0], self.data[node.pair.1], node.d);
            // print_cluster(&clusters);
            // println!("\n\n");
            if clusters.len() == 1 {
                let p1 = &self.data[node.pair.0];
                let p2 = &self.data[node.pair.1];
                return (p1.x * p2.x ) as u64;
            }
            
        }
        0
    }
}
