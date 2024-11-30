use std::collections::HashSet;
use std::fs;
use crate::traits::Day;

pub struct Day22 {
    bricks: Vec<([usize; 3], [usize; 3])>
}

impl Day22 {
    pub fn new() -> Self {
        let mut bricks = fs::read_to_string("data/year2023/day22")
            .expect("Cannot read file")
            .split('\n')
            .map(|line| {
                let (start, end) = line.split_once('~').expect("Line malformed");
                (Day22::parse_point(start).unwrap_or_else(|err| panic!("Error: {err}")),
                 Day22::parse_point(end).unwrap_or_else(|err| panic!("Error: {err}")))
            }).collect::<Vec<_>>();

        bricks.sort_by(|a, b| a.0[2].cmp(&b.0[2]));
        Day22 {
            bricks
        }
    }

    pub fn parse_point(point_str: &str) -> Result<[usize; 3], String> {
        let parts:Vec<&str> = point_str.split(",").collect();
        if parts.len() != 3 {
            Err(format!("String {point_str} does not have 3 points"))
        } else {
            let mut points = [0; 3];
            for (i, &point) in parts.iter().enumerate() {
                match point.parse::<usize>() {
                    Ok(num) => points[i] = num,
                    Err(_) => return Err(format!("Point {point} is not a valid number"))
                }
            }

            Ok(points)
        }
    }

    fn settle(&mut self) {
        let roof =  self.bricks[self.bricks.len()-1].0[2];
        let mut occupied_tiles: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(); roof];
        for level in 1..=roof {
            self.bricks.iter_mut().filter(|x| x.0[2] == level)
                .for_each(|(start, end)| {
                    //         check the lowest level that it fits
                    let mut settle_floor = level - 1;
                    for (i, floor) in occupied_tiles[0..level].iter_mut().enumerate().rev() {
                        if !Day22::can_brick_fit_in_floor(floor, start, end) {
                            break;
                        }
                        settle_floor = i;
                    }
                    let jump = start[2] - settle_floor;
                    start[2] -= jump;
                    end[2] -= jump;

                    //         update occupied_tiles
                    for tile in occupied_tiles[start[2]..=end[2]].iter_mut() {
                        for y in start[1]..=end[1] {
                            for x in start[0]..=end[0] {
                                tile.insert((x, y));
                            }
                        }
                    }
                });
        }
    }

    pub fn can_brick_fit_in_floor(floor: &HashSet<(usize, usize)>, start:&[usize; 3], end:&[usize; 3]) -> bool {
        for x in start[0]..=end[0] {
            for y in start[1]..=end[1] {
                if floor.contains(&(x, y)) {
                    return false;
                }
            }
        }
        true
    }

    fn is_only_leaf(graph: &Vec<Vec<usize>>, start: usize, leaf: usize) -> bool {
        let mut stack = vec![start];
        let mut leafs: HashSet<usize> = HashSet::new();
        while let Some(node) = stack.pop() {
            if graph[node].is_empty() || node == leaf {
                leafs.insert(node);
                continue;
            }
            for g in &graph[node] {
                stack.push(*g);
            }
        }
        leafs.len() == 1 && leafs.contains(&leaf)
    }

    fn count_nodes_destroyed(graph: &Vec<Vec<usize>>, rev_graph: &[Vec<usize>], node: usize) -> u64 {
        let mut stack = vec![node];
        let mut nodes: HashSet<usize> = HashSet::new();
        while let Some(curr) = stack.pop() {
            nodes.insert(curr);
            for nxt in &rev_graph[curr] {
                if Day22::is_only_leaf(graph, curr, node) {
                    stack.push(*nxt);
                }
            }
        }

        (nodes.len() - 1) as u64
    }

}

impl Day for Day22 {
    fn part_1(&mut self) -> u64 {
        self.settle();
        let count = self.bricks.len();
        let mut brick_index:HashSet<usize> = HashSet::new();

        for (start, end) in self.bricks.iter().rev() {
            if start[2] == 0 {
                continue;
            }
            let mut points = vec![];
            for x in start[0]..=end[0] {
                for y in start[1]..=end[1] {
                    points.push((x, y));
                }
            }
            let floor_below = start[2] - 1;
            let supporting_bricks = self.bricks.iter().enumerate()
                .filter(|(_, x)| x.0[2] == floor_below || x.1[2] == floor_below)
                .filter_map(|(i, (start, end))| {
                    for x in start[0]..=end[0] {
                        for y in start[1]..=end[1] {
                            if points.contains(&(x, y)) {
                                return Some(i)
                            }
                        }
                    }
                   None
                }).collect::<Vec<_>>();
            if supporting_bricks.len() == 1 {
                brick_index.insert(supporting_bricks[0]);
            }
        }

        (count - brick_index.len()) as u64
    }

    fn part_2(&mut self) -> u64 {
        self.settle();
        let mut brick_index:HashSet<usize> = HashSet::new();
        let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; self.bricks.len()];
        for (i, (start, end)) in self.bricks.iter().enumerate().rev() {
            if start[2] == 0 {
                continue;
            }
            let mut points = vec![];
            for x in start[0]..=end[0] {
                for y in start[1]..=end[1] {
                    points.push((x, y));
                }
            }
            let floor_below = start[2] - 1;
            let supporting_bricks = self.bricks.iter().enumerate()
                .filter(|(_, x)| x.0[2] == floor_below || x.1[2] == floor_below)
                .filter_map(|(i, (start, end))| {
                    for x in start[0]..=end[0] {
                        for y in start[1]..=end[1] {
                            if points.contains(&(x, y)) {
                                return Some(i)
                            }
                        }
                    }
                    None
                }).collect::<Vec<_>>();
            if supporting_bricks.len() == 1 {
                brick_index.insert(supporting_bricks[0]);
            }
            adjacency_list[i] = supporting_bricks;
        }
        let mut reverse_graph:Vec<Vec<usize>> = vec![vec![]; self.bricks.len()];
        for (i, adj) in adjacency_list.iter().enumerate() {
            for a in adj {
                reverse_graph[*a].push(i);
            }
        }
        let mut count = 0;
        for i in brick_index {
            let res = Day22::count_nodes_destroyed(&adjacency_list, &reverse_graph, i);
            count += res;
        }
        count
    }
}
