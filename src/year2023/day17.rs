use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use crate::traits::Day;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Direction {
    South,
    North,
    East,
    West
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        matches!(self, Direction::West | Direction::East)
    }
}

impl Direction {
    fn repr(&self) -> String {
        match self {
            Direction::South => {
                String::from("\x1b[31mv\x1b[0m")
            }
            Direction::North => {
                String::from("\x1b[31m^\x1b[0m")
            }
            Direction::East => {
                String::from("\x1b[31m>\x1b[0m")
            }
            Direction::West => {
                String::from("\x1b[31m<\x1b[0m")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone)]
struct Node {
    location: Point,
    weight: u32,
    direction: Direction,
    count: u8,
    previous: Option<Box<Node>>
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.hash(state);
        self.direction.is_horizontal().hash(state);
        self.count.hash(state);
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.direction.is_horizontal() == other.direction.is_horizontal() && self.count == other.count
    }
}

impl Eq for Node {}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

pub struct Day17 {
    graph: Vec<Vec<u32>>
}

impl Day17 {
    pub fn new() -> Self {
        let graph = fs::read_to_string("data/day17")
            .expect("Cannot read data")
            .split('\n')
            .map(|x|
                x.chars()
                    .map(|num| num.to_digit(10).expect("A non digit value found"))
                    .collect::<Vec<_>>()
            ).collect::<Vec<_>>();

        Day17 {
            graph
        }
    }

    fn dijkstra_shortest_path(&self, max_step_limit: u8, min_step_limit: u8) -> u64 {
        let mut heap:BinaryHeap<Node> = BinaryHeap::new();
        let x_lim = self.graph.len();
        let y_lim = self.graph[0].len();
        let end_location = Point::new(x_lim - 1, y_lim - 1);
        let mut visited:HashSet<Node> = HashSet::new();
        let start_node = Node {
            location: Point::new(0, 0),
            weight: 0,
            direction: Direction::East,
            count: 0,
            previous: None
        };
        heap.push(start_node);
        let mut end_node = None;
        while let Some(node) = heap.pop() {
            if node.location == end_location {
                if node.count < min_step_limit {
                    continue;
                }
                end_node = Some(node);
                break;
            }
            if visited.contains(&node) {
                continue;
            }
            // east
            if node.location.y < y_lim - 1 && node.direction != Direction::West {
                if node.direction == Direction::East {
                    if node.count < max_step_limit {
                        let next_node = Node {
                            location: Point::new(node.location.x, node.location.y+1),
                            weight: node.weight + self.graph[node.location.x][node.location.y+1],
                            direction: Direction::East,
                            count: node.count + 1,
                            previous: Some(Box::new(node.clone()))
                        };
                        heap.push(next_node);
                    }
                } else if node.count >= min_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.x, node.location.y+1),
                        weight: node.weight + self.graph[node.location.x][node.location.y+1],
                        direction: Direction::East,
                        count: 1,
                        previous: Some(Box::new(node.clone()))
                    };
                    heap.push(next_node);
                }
            }
            // west
            if node.location.y >= 1 && node.direction != Direction::East {
                if node.direction == Direction::West {
                    if node.count < max_step_limit {
                        let next_node = Node {
                            location: Point::new(node.location.x, node.location.y-1),
                            weight: node.weight + self.graph[node.location.x][node.location.y-1],
                            direction: Direction::West,
                            count: node.count + 1,
                            previous: Some(Box::new(node.clone()))
                        };
                        heap.push(next_node);
                    }
                } else if node.count >= min_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.x, node.location.y-1),
                        weight: node.weight + self.graph[node.location.x][node.location.y-1],
                        direction: Direction::West,
                        count: 1,
                        previous: Some(Box::new(node.clone()))
                    };
                    heap.push(next_node);
                }
            }
            // north
            if node.location.x >= 1 && node.direction != Direction::South {
                if node.direction == Direction::North {
                    if node.count < max_step_limit {
                        let next_node = Node {
                            location: Point::new(node.location.x-1, node.location.y),
                            weight: node.weight + self.graph[node.location.x-1][node.location.y],
                            direction: Direction::North,
                            count: node.count+1,
                            previous: Some(Box::new(node.clone()))
                        };
                        heap.push(next_node);
                    }
                } else if node.count >= min_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.x-1, node.location.y),
                        weight: node.weight + self.graph[node.location.x-1][node.location.y],
                        direction: Direction::North,
                        count: 1,
                        previous: Some(Box::new(node.clone()))
                    };
                    heap.push(next_node);
                }
            }
            // south
            if node.location.x < x_lim - 1 && node.direction != Direction::North {
                if node.direction == Direction::South {
                    if node.count < max_step_limit {
                        let next_node = Node {
                            location: Point::new(node.location.x+1, node.location.y),
                            weight: node.weight + self.graph[node.location.x+1][node.location.y],
                            direction: Direction::South,
                            count: node.count+1,
                            previous: Some(Box::new(node.clone()))
                        };
                        heap.push(next_node);
                    }
                } else if node.count >= min_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.x+1, node.location.y),
                        weight: node.weight + self.graph[node.location.x+1][node.location.y],
                        direction: Direction::South,
                        count: 1,
                        previous: Some(Box::new(node.clone()))
                    };
                    heap.push(next_node);
                }
            }
            visited.insert(node);
        }


        if let Some(node) = end_node {
            let mut data = self.graph
                .iter()
                .map(|row| row
                    .iter()
                    .map(|val| val.to_string())
                    .collect::<Vec<_>>()
                ).collect::<Vec<_>>();
            let mut curr = &Box::new(node.clone());
            data[curr.location.x][curr.location.y] = curr.direction.repr();
            while let Some(prev) = &curr.previous {
                let dir = &prev.direction;
                if !(prev.location.x == 0 && prev.location.y == 0) {
                    data[prev.location.x][prev.location.y] = dir.repr();
                }
                curr = prev;
            }

            for x in data {
                for y in x {
                    print!("{y}");
                }
                println!();
            }

            node.weight as u64
        } else {
            panic!("Cannot reach end in the provided graph");
        }
    }
}

impl Day for Day17 {
    fn part_1(&self) -> u64 {
        self.dijkstra_shortest_path(3, 0)
    }

    fn part_2(&self) -> u64 {
        self.dijkstra_shortest_path(10, 4)
    }
}
