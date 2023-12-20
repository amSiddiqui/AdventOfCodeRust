use std::collections::{HashMap, HashSet};
use std::fs;
use crate::traits::Day;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y}
    }

    fn left(&self) -> Point {
        Point::new(self.x, self.y-1)
    }
    fn right(&self) -> Point {
        Point::new(self.x, self.y+1)
    }

    fn up(&self) -> Point {
        Point::new(self.x-1, self.y)
    }

    fn down(&self) -> Point {
        Point::new(self.x+1, self.y)
    }
}

pub struct Day18 {
    lines: Vec<(char, u8, String)>
}

impl Day18 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day18")
            .expect("Cannot read file")
            .split('\n')
            .map(|line| {
                let mut parts = line
                    .split_whitespace();
                let dir = parts.next()
                    .expect("Direction not provided")
                    .as_bytes()[0] as char;
                let length = parts.next()
                    .expect("Length now provided")
                    .parse::<u8>()
                    .expect("Cannot parse length, invalid character");
                let color = parts.next()
                    .expect("Color not provided");
                if color.chars().count() <= 3 {
                    panic!("Empty color provided {color}");
                }
                let color = String::from(&color[2..color.len() - 1]);
                (dir, length, color)
            }).collect::<Vec<_>>();

        Day18 {
            lines
        }
    }

    fn get_all_vertex_and_vertical_edge_start(&self) -> (HashSet<Point>, HashSet<Point>, Point, Point) {
        let mut vertex = HashSet::new();
        let mut vertical_start = HashSet::new();
        let mut smallest = Point::new(0, 0);
        let mut largest = Point::new(0, 0);

        let mut pos = Point::new(0, 0);
        vertex.insert(pos);

        for (dir, length, _) in &self.lines {
            if *dir == 'L' {
                for _ in 0..*length {
                    pos = pos.left();
                    vertex.insert(pos);
                }
                if pos.y < smallest.y {
                    smallest.y = pos.y;
                }
            } else if *dir == 'R' {
                for _ in 0..*length {
                    pos = pos.right();
                    vertex.insert(pos);
                }
                if pos.y > largest.y {
                    largest.y = pos.y;
                }
            } else if *dir == 'U' {
                for _ in 0..*length {
                    pos = pos.up();
                    vertical_start.insert(pos);
                    vertex.insert(pos);
                }
                if pos.x < smallest.x {
                    smallest.x = pos.x;
                }
            } else if *dir == 'D' {
                vertical_start.insert(pos);
                for _ in 0..length-1 {
                    pos = pos.down();
                    vertical_start.insert(pos);
                    vertex.insert(pos);
                }
                pos = pos.down();
                vertex.insert(pos);
                if pos.x > largest.x {
                    largest.x = pos.x;
                }
            }
        }
        (vertex, vertical_start, smallest, largest)
    }

    fn check_vertex_inside_box(point: &Point, vertical_vertex: &HashSet<Point>, y_lim: i32) -> bool {
        let mut current= *point;
        let mut count = 0;
        for _ in point.y+1..y_lim {
            current = current.right();
            if vertical_vertex.contains(&current) {
                count += 1;
            }
        }
        count % 2 == 1
    }
}

impl Day for Day18 {
    fn part_1(&mut self) -> u64 {
        let (vertex, vertical_vertex, smallest, largest) = self.get_all_vertex_and_vertical_edge_start();
        let mut count = 0;
        println!("Smallest: {smallest:?} ;; Largest: {largest:?}");
        for x in smallest.x..=largest.x {
            for y in smallest.y..=largest.y {
                let p = Point::new(x, y);
                if vertex.contains(&p) || Day18::check_vertex_inside_box(&p, &vertical_vertex, largest.y+1) {
                    count += 1;
                }
            }
        }

        count
    }

    fn part_2(&mut self) -> u64 {
        let mut current = Point::new(0, 0);
        let mut vertical_edges: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut left_edge_sum = 0_u64;
        for (_, _, hex) in &self.lines {
            let num_str = &hex.as_str()[0..hex.len()-1];
            let num = i32::from_str_radix(num_str, 16)
                .unwrap_or_else(|err| {
                    panic!("Invalid hex string found {num_str};; Err {err}");
                });
            let dir = hex.as_bytes()[hex.len()-1] as char;
            if dir == '0' {
                current.y += num;
            } else if dir == '2' {
                current.y -= num;
                left_edge_sum += num as u64;
            } else if dir == '1' {
                //     down
                for i in current.x..current.x+num {
                    vertical_edges.entry(i).or_default().push(current.y);
                }
                current.x += num;
            } else if dir == '3' {
                //     up
                for i in current.x-num..current.x {
                    vertical_edges.entry(i).or_default().push(current.y);
                }
                current.x -= num;
            }
        }
        let mut sum = 0_u64;
        for (_, y) in vertical_edges.iter_mut() {
            y.sort();
            for chunk in y.chunks(2) {
                if chunk.len() != 2 {
                    panic!("Uneven pairs of coordinates");
                }
                sum += (chunk[1] - chunk[0] + 1) as u64;
            }
        }
        sum + left_edge_sum + 1
    }
}
