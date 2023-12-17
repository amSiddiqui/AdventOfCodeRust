use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use crate::traits::Day;

#[derive(Eq, PartialEq, Debug, Clone, Hash, Copy)]
enum Direction {
    South,
    North,
    East,
    West
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Node {
    location: Point,
    weight: u32,
    direction: Direction,
    count: u8
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.location.hash(state);
        self.direction.hash(state);
        self.count.hash(state);
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.direction == other.direction && self.count == other.count
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
        let mut visited:HashSet<(Point, Direction, u8)> = HashSet::new();
        let start_node = Node {
            location: Point::new(0, 0),
            weight: 0,
            direction: Direction::East,
            count: 0
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
            if visited.contains(&(node.location, node.direction, node.count)) {
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
                        };
                        heap.push(next_node);
                    }
                } else if node.count >= min_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.x, node.location.y+1),
                        weight: node.weight + self.graph[node.location.x][node.location.y+1],
                        direction: Direction::East,
                        count: 1
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
                            count: node.count + 1
                        };
                        heap.push(next_node);
                    }
                } else if node.count >= min_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.x, node.location.y-1),
                        weight: node.weight + self.graph[node.location.x][node.location.y-1],
                        direction: Direction::West,
                        count: 1
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
                            count: node.count+1
                        };
                        heap.push(next_node);
                    }
                } else if node.count >= min_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.x-1, node.location.y),
                        weight: node.weight + self.graph[node.location.x-1][node.location.y],
                        direction: Direction::North,
                        count: 1
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
                            count: node.count+1
                        };
                        heap.push(next_node);
                    }
                } else if node.count >= min_step_limit {
                    let next_node = Node {
                        location: Point::new(node.location.x+1, node.location.y),
                        weight: node.weight + self.graph[node.location.x+1][node.location.y],
                        direction: Direction::South,
                        count: 1
                    };
                    heap.push(next_node);
                }
            }
            visited.insert((node.location, node.direction, node.count));
        }
        drop(visited);

        if let Some(node) = end_node {
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
