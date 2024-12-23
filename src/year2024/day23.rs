use std::fs;

use ahash::{HashMap, HashSet};

use crate::traits::Day;

pub struct Day23 {
    connections: Vec<(String, String)>,
}

impl Day23 {
    pub fn new() -> Self {
        let connections = fs::read_to_string("data/year2024/day23")
            .expect("Cannot read data file")
            .lines()
            .map(|l| {
                let mut parts = l.split('-');
                let a = parts.next().unwrap().to_string();
                let b = parts.next().unwrap().to_string();
                (a, b)
            })
            .collect();
        Day23 { connections }
    }
}

impl Day for Day23 {
    fn part_1(&mut self) -> u64 {
        let mut connection_list: HashMap<String, HashSet<String>> = HashMap::default();
        for (a, b) in &self.connections {
            connection_list
                .entry(a.clone())
                .or_default()
                .insert(b.clone());
            connection_list
                .entry(b.clone())
                .or_default()
                .insert(a.clone());
        }

        let mut sets_of_3: HashSet<[String; 3]> = HashSet::default();
        for (a, b) in &self.connections {
            for c in connection_list.get(a).unwrap() {
                if connection_list.get(c).unwrap().contains(b) {
                    // sort a, b and c before inserting
                    let mut a = a.clone();
                    let mut b = b.clone();
                    let mut c = c.clone();
                    let mut temp;
                    if a > b {
                        temp = a;
                        a = b;
                        b = temp;
                    }
                    if b > c {
                        temp = b;
                        b = c;
                        c = temp;
                    }
                    if a > b {
                        temp = a;
                        a = b;
                        b = temp;
                    }
                    sets_of_3.insert([a.clone(), c.clone(), b.clone()]);
                }
            }
        }

        let mut count = 0;
        for [a, b, c] in sets_of_3 {
            if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
                count += 1;
            }
        }

        count
    }

    fn part_2(&mut self) -> u64 {
        let mut connection_list: HashMap<String, HashSet<String>> = HashMap::default();
        for (a, b) in &self.connections {
            connection_list
                .entry(a.clone())
                .or_default()
                .insert(b.clone());
            connection_list
                .entry(b.clone())
                .or_default()
                .insert(a.clone());
        }

        for (key, connections) in connection_list.clone() {
            for conn in connections {
                connection_list.entry(conn).or_default().insert(key.clone());
            }
        }

        let mut largest_interconnected_list: Vec<String> = vec![];

        for (a, b) in &self.connections {
            let mut interconnected_list = HashSet::default();
            interconnected_list.insert(a.clone());
            interconnected_list.insert(b.clone());

            let connections_a = connection_list.get(a).unwrap();
            for c in connections_a {
                let connections_c = connection_list.get(c).unwrap();
                if interconnected_list.is_subset(connections_c) {
                    interconnected_list.insert(c.clone());
                }
            }

            if interconnected_list.len() > largest_interconnected_list.len() {
                largest_interconnected_list = interconnected_list.iter().cloned().collect();
            }
        }

        largest_interconnected_list.sort();
        let password = largest_interconnected_list.join(",");
        println!("{}", password);

        0
    }
}
