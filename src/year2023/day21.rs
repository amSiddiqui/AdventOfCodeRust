use std::collections::{HashSet, VecDeque};
use std::fs;
use std::hash::{Hash, Hasher};
use crate::traits::Day;

pub struct Day21 {
    lines: Vec<Vec<char>>,
    x_lim: i32,
    y_lim: i32,
    start: Point
}



#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32,
    distance: u32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Point {
    fn new(x: i32, y: i32, distance: u32) -> Self {
        Point { x, y, distance }
    }
}

impl Day21 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day21")
            .expect("Cannot read file")
            .split('\n')
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut x = 0;
        let mut y = 0;
        let mut found = false;

        for (i, row) in lines.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == 'S' {
                    x = i;
                    y = j;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        let start = Point::new(x as i32, y as i32, 0);
        let x_lim = lines.len() as i32;
        let y_lim = lines[0].len() as i32;

        Day21 {
            lines,
            start,
            x_lim,
            y_lim
        }
    }

    pub fn get_val(&self, mut x:i32, mut y:i32) -> char {
        x %= self.x_lim;
        y %= self.y_lim;
        if x < 0 {
            x += self.x_lim;
        }
        if y < 0 {
            y += self.y_lim;
        }
        self.lines[x as usize][y as usize]
    }

    pub fn walk(&mut self, limit: u32) -> u64 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.start);
        let mut count = 0;

        while let Some(mut node) = queue.pop_front() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            if node.distance == limit {
                count += 1;
                continue;
            } else if (limit - node.distance) % 2 == 0 {
                count += 1;
            }
            node.distance += 1;
            let mut points = vec![node; 4];
            points[0].x -= 1;
            points[1].x += 1;
            points[2].y += 1;
            points[3].y -= 1;
            for next_point in points {
                if self.get_val(next_point.x, next_point.y) != '#' {
                    queue.push_back(next_point);
                }
            }
        }
        count
    }
}

impl Day for Day21 {
    fn part_1(&mut self) -> u64 {
        self.walk(64)
    }

    fn part_2(&mut self) -> u64 {
        self.walk(26501365)
    }
}
