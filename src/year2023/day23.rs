use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::fs;
use std::hash::{Hash, Hasher};
use crate::traits::Day;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    West,
    East,
    North,
    South
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    direction: Direction,
    distance: usize
}

impl Point {
    fn can_go_north(&self) -> bool {
        self.x > 0
    }

    fn can_go_west(&self) -> bool {
        self.y > 0
    }

    fn can_go_east(&self, y_lim: usize) -> bool {
        self.y < y_lim - 1
    }

    fn can_go_south(&self, x_lim: usize) -> bool {
        self.x < x_lim - 1
    }

    fn go_north(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
            direction: Direction::North,
            distance: self.distance + 1
        }
    }

    fn go_south(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
            direction: Direction::South,
            distance: self.distance + 1
        }
    }

    fn go_west(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
            direction: Direction::West,
            distance: self.distance + 1
        }
    }

    fn go_east(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
            direction: Direction::East,
            distance: self.distance + 1
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

pub struct Day23 {
    graph: Vec<Vec<char>>,
    x_lim: usize,
    y_lim: usize
}

impl Day23 {
    pub fn new() -> Self {
        let graph = fs::read_to_string("data/day23")
            .expect("Cannot read file")
            .split('\n')
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let x_lim = graph.len();
        let y_lim = graph[0].len();
        Day23 {
            graph,
            x_lim,
            y_lim
        }
    }

    fn walk(&self) -> usize {
        let mut heap: BinaryHeap<Point> = BinaryHeap::new();
        let start = Point {x: 0, y: 1, direction: Direction::South, distance: 0};
        let end = Point { x: self.x_lim - 1, y: self.y_lim - 2, direction: Direction::South, distance: 0 };
        heap.push(start);
        let mut res = 0;
        while let Some(point) = heap.pop() {
            if point == end {
                if point.distance > res {
                    res = point.distance;
                }
                continue;
            }
            let slope = match self.graph[point.x][point.y] {
                '>' => Some(Direction::East),
                '<' => Some(Direction::West),
                'v' => Some(Direction::South),
                '^' => Some(Direction::North),
                _ => None
            };
        //     go south
            if point.can_go_south(self.x_lim)
                && point.direction != Direction::North
                && (slope.is_none() || slope.unwrap() == Direction::South)
            {
                let next_point = point.go_south();
                let tile = self.graph[next_point.x][next_point.y];
                if tile != '#' && tile != '^' {
                    heap.push(next_point);
                }
            }

        //     go north
            if point.can_go_north()
                && point.direction != Direction::South
                && (slope.is_none() || slope.unwrap() == Direction::North)
            {
                let next_point = point.go_north();
                let tile = self.graph[next_point.x][next_point.y];
                if tile != '#' && tile != 'v' {
                    heap.push(next_point);
                }
            }

        //     go east
            if point.can_go_east(self.y_lim)
                && point.direction != Direction::West
                && (slope.is_none() || slope.unwrap() == Direction::East)
            {
                let next_point = point.go_east();
                let tile = self.graph[next_point.x][next_point.y];
                if tile != '#' && tile != '<' {
                    heap.push(next_point);
                }
            }

        //     go west
            if point.can_go_west()
                && point.direction != Direction::East
                && (slope.is_none() || slope.unwrap() == Direction::West)
            {
                let next_point = point.go_west();
                let tile = self.graph[next_point.x][next_point.y];
                if tile != '#' && tile != '>' {
                    heap.push(next_point);
                }
            }
        }
        res
    }

}


impl Day for Day23 {
    fn part_1(&mut self) -> u64 {
        self.walk() as u64
    }

    fn part_2(&mut self) -> u64 {
        let grid = &self.graph;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut max_path_len = 0;

        fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, row: usize, col: usize, current_len: usize, max_path_len: &mut usize) {
            let rows = grid.len();
            let cols = grid[0].len();
            if row == rows - 1 && col == cols - 2 {
                *max_path_len = usize::max(*max_path_len, current_len);
                return;
            }

            let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for (dr, dc) in directions.iter() {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;

                if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    if grid[new_row][new_col] != '#' && !visited[new_row][new_col] {
                        visited[new_row][new_col] = true;
                        dfs(grid, visited, new_row, new_col, current_len + 1, max_path_len);
                        visited[new_row][new_col] = false;
                    }
                }
            }
        }

        if grid[0][1] != '#' {
            visited[0][1] = true;
            dfs(grid, &mut visited, 0, 1, 0, &mut max_path_len);
        }

        max_path_len as u64
    }
}
