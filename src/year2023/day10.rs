use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::iproduct;
use crate::traits::Day;
use rayon::prelude::*;

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn south_move(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y
        }
    }

    fn north_move(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y
        }
    }

    fn east_move(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1
        }
    }

    fn west_move(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1
        }
    }
}

#[derive(Debug)]
struct Edge {
    start: Point,
    end: Point,
}

impl Edge {
    fn new(p1: Point, p2: Point) -> Self {
        let mut start = p1;
        let mut end = p2;
        if p1.y == p2.y && p1.x > p2.x {
            start = p2;
            end = p1;
        }

        Edge {
            start, end
        }
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }
}


pub struct Day10 {
    lines: Vec<String>,
    possible_moves: HashMap<char, (Direction, Direction)>,
    x_limit: usize,
    y_limit: usize
}


impl Day10 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day10")
            .expect("Cannot read input file")
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>();

        let x_limit = lines.len();
        let y_limit = lines[0].len();

        Day10 {
            lines,
            possible_moves: Day10::get_possible_moves(),
            x_limit,
            y_limit
        }
    }

    fn can_move(tile: char, direction: Direction) -> bool {
        match direction {
            Direction::North => {
                "|F7S".find(tile).is_some()
            },
            Direction::South => {
                "|LJS".find(tile).is_some()
            },
            Direction::East => {
                "-J7S".find(tile).is_some()
            },
            Direction::West => {
                "-FLS".find(tile).is_some()
            }
        }
    }

    fn get_possible_moves() -> HashMap<char, (Direction, Direction)> {
        let mut map = HashMap::new();
        map.insert('|', (Direction::North, Direction::South));
        map.insert('-', (Direction::East, Direction::West));
        map.insert('L', (Direction::North, Direction::East));
        map.insert('J', (Direction::North, Direction::West));
        map.insert('7', (Direction::West, Direction::South));
        map.insert('F', (Direction::East, Direction::South));
        map
    }

    fn get_starting_location(&self) -> Option<Point> {
        for (x, line) in self.lines.iter().enumerate() {
            for (y, tile) in line.chars().enumerate() {
                if tile == 'S' {
                    return Some(Point { x, y });
                }
            }
        }
        None
    }

    fn get_tile(&self, point: Point) -> char {
        self.lines[point.x].as_bytes()[point.y] as char
    }

    fn traverse(&self, point: Point, mut start: Direction) -> (HashSet<Point>, Vec<Edge>) {
        let mut c_tile = self.get_tile(point);
        let mut c_point = point;
        let mut visited = HashSet::new();
        let mut edges: Vec<Edge> = Vec::new();

        while c_tile != 'S' && !visited.contains(&c_point) {
            visited.insert(c_point);
            if let Some((dir1, dir2)) = self.possible_moves.get(&c_tile) {
                let mut next_dir = dir1;
                if start == *dir1 {
                    next_dir = dir2
                }
                let mut should_move = false;
                match next_dir {
                    Direction::South => {
                        if c_point.x < self.x_limit
                            && Day10::can_move(self.get_tile(c_point.south_move()),
                                               Direction::South) {
                            let south_move = c_point.south_move();
                            edges.push(Edge::new(c_point, south_move));
                            c_point = south_move;
                            start = Direction::North;
                            should_move = true;
                        }
                    },
                    Direction::North => {
                        if c_point.x > 0
                            && Day10::can_move(self.get_tile(c_point.north_move()),
                                               Direction::North) {
                            let north_move = c_point.north_move();
                            edges.push(Edge::new(c_point, north_move));
                            c_point = north_move;
                            start = Direction::South;
                            should_move = true;
                        }
                    },
                    Direction::West => {
                        if c_point.y > 0
                            && Day10::can_move(self.get_tile(c_point.west_move()),
                                               Direction::West) {
                            let west_move = c_point.west_move();
                            edges.push(Edge::new(c_point, west_move));
                            c_point = west_move;
                            start = Direction::East;
                            should_move = true;
                        }
                    },
                    Direction::East => {
                        if c_point.y < self.y_limit
                            && Day10::can_move(self.get_tile(c_point.east_move()),
                                               Direction::East) {
                            let east_move = c_point.east_move();
                            edges.push(Edge::new(c_point, east_move));
                            c_point = east_move;
                            start = Direction::West;
                            should_move = true;
                        }
                    }
                }
                if !should_move {
                    println!("Reached dead end");
                    break;
                } else {
                    c_tile = self.get_tile(c_point);
                }
            } else {
                println!("No possible moves. Reached dead end");
                break;
            }
        }
        (visited, edges)
    }

    fn check_point_inside_path(&self, point: Point, edges: &Vec<Edge>) -> bool {
        // Cast a ray from point to right edge
        // TODO: A lot of scope of optimization here
        let mut intersection = 0;
        for y in point.y+1..self.y_limit {
            let next_point = Point { x: point.x, y };
            for edge in edges {
                if edge.start == next_point {
                    intersection += 1;
                    break;
                }
            }
        }
        intersection % 2 == 1
    }
}

impl Day for Day10 {
    fn part_1(&self) -> u64 {
        let starting_location = self.get_starting_location()
            .expect("Starting position not provided");

        let south_start = starting_location.south_move();
        let north_start = starting_location.north_move();
        let east_start = starting_location.east_move();
        let west_start = starting_location.west_move();

        let mut max_visited = 0;
        let (south_path, _) = self.traverse(south_start, Direction::North);
        if max_visited < south_path.len() {
            max_visited = south_path.len();
        }
        let (north_paths, _) = self.traverse(north_start, Direction::South);
        if max_visited < north_paths.len() {
            max_visited = north_paths.len();
        }
        let (west_paths, _) = self.traverse(west_start, Direction::West);
        if max_visited < west_paths.len() {
            max_visited = west_paths.len();
        }
        let (east_paths, _) = self.traverse(east_start, Direction::West);
        if max_visited < east_paths.len() {
            max_visited = east_paths.len();
        }
        (max_visited as u64 + 1) / 2_u64
    }

    fn part_2(&self) -> u64 {
        let starting_location = self.get_starting_location()
            .expect("Starting position not provided");
        let south_start = starting_location.south_move();
        let (mut path, edges) = self.traverse(south_start, Direction::North);
        let mut edges = edges.into_iter().filter(|x| x.is_vertical()).collect::<Vec<_>>();
        path.insert(starting_location);
        edges.push(Edge::new(starting_location, south_start));
        let all_points = iproduct!(0..self.x_limit, 0..self.y_limit);
        all_points
            .par_bridge()
            .filter(|&(x, y)| {
                let point = Point { x, y };
                !path.contains(&point) && self.check_point_inside_path(point, &edges)
            }).count() as u64
    }
}

