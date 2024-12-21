use std::{fs, u64};

use ahash::{HashMap, HashSet};

use crate::traits::Day;

pub struct Day20 {
    graph: Vec<Vec<char>>,
    start: (usize, usize),
    x_lim: usize,
    y_lim: usize,
}

fn find_char(graph: &Vec<Vec<char>>, ch: char) -> Option<(usize, usize)> {
    for (y, line) in graph.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == ch {
                return Some((x, y));
            }
        }
    }
    None
}

impl Day20 {
    pub fn new() -> Self {
        let graph: Vec<Vec<char>> = fs::read_to_string("data/year2024/day20")
            .expect("Cannot read data file")
            .lines()
            .map(|c| c.chars().collect())
            .collect();

        let start = find_char(&graph, 'S').expect("Cannot find S");
        let x_lim = graph[0].len() - 1;
        let y_lim = graph.len() - 1;

        Day20 {
            graph,
            start,
            x_lim,
            y_lim,
        }
    }

    fn find_cheat_walls(&self) -> HashSet<(usize, usize)> {
        let mut walls_h = HashSet::default();

        for (y, line) in self.graph.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if *ch != '#' {
                    continue;
                }

                if x > 0
                    && x < self.x_lim
                    && self.graph[y][x - 1] != '#'
                    && self.graph[y][x + 1] != '#'
                {
                    walls_h.insert((x, y));
                }

                if y > 0
                    && y < self.y_lim
                    && self.graph[y + 1][x] != '#'
                    && self.graph[y - 1][x] != '#'
                {
                    walls_h.insert((x, y));
                }
            }
        }

        walls_h
    }

    fn calculate_path_score(
        &mut self,
        score: u64,
        x: usize,
        y: usize,
        score_path: &mut HashMap<(usize, usize), u64>,
        max_score: u64,
    ) {
        if score == 0 {
            return;
        }

        score_path.insert((x, y), max_score - score);
        self.graph[y][x] = 'E';

        if x > 0 && self.graph[y][x - 1] == '.' {
            self.calculate_path_score(score - 1, x - 1, y, score_path, max_score);
        }

        if x < self.graph[0].len() && self.graph[y][x + 1] == '.' {
            self.calculate_path_score(score - 1, x + 1, y, score_path, max_score);
        }

        if y > 0 && self.graph[y - 1][x] == '.' {
            self.calculate_path_score(score - 1, x, y - 1, score_path, max_score);
        }

        if y < self.graph.len() && self.graph[y + 1][x] == '.' {
            self.calculate_path_score(score - 1, x, y + 1, score_path, max_score);
        }
    }

    fn full_path(&self, x: usize, y: usize, dir: char, path: &mut Vec<(usize, usize)>) {
        if self.graph[y][x] == 'E' {
            return;
        }

        path.push((x, y));

        if dir != 'E' && x > 0 && self.graph[y][x - 1] != '#' {
            self.full_path(x - 1, y, 'W', path);
        }

        if dir != 'W' && x < self.x_lim && self.graph[y][x + 1] != '#' {
            self.full_path(x + 1, y, 'E', path);
        }

        if dir != 'S' && y > 0 && self.graph[y - 1][x] != '#' {
            self.full_path(x, y - 1, 'N', path);
        }

        if dir != 'N' && y < self.y_lim && self.graph[y + 1][x] != '#' {
            self.full_path(x, y + 1, 'S', path);
        }
    }
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> u64 {
    ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as u64
}

pub fn dfs(
    graph: &Vec<Vec<char>>,
    score: u64,
    x: usize,
    y: usize,
    x_lim: usize,
    y_lim: usize,
    score_lim: u64,
    visited: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if score > score_lim {
        return u64::MAX;
    }

    if graph[y][x] == 'E' {
        return score;
    }

    if let Some(v_score) = visited.get(&(x, y)) {
        if *v_score < score {
            return u64::MAX;
        }
    }

    visited.insert((x, y), score);

    let mut smallest_score = u64::MAX;

    if x > 0 && graph[y][x - 1] != '#' {
        let ns = dfs(graph, score + 1, x - 1, y, x_lim, y_lim, score_lim, visited);
        if ns < smallest_score {
            smallest_score = ns;
        }
    }

    if x < x_lim && graph[y][x + 1] != '#' {
        let ns = dfs(graph, score + 1, x + 1, y, x_lim, y_lim, score_lim, visited);
        if ns < smallest_score {
            smallest_score = ns;
        }
    }

    if y > 0 && graph[y - 1][x] != '#' {
        let ns = dfs(graph, score + 1, x, y - 1, x_lim, y_lim, score_lim, visited);
        if ns < smallest_score {
            smallest_score = ns;
        }
    }

    if y < y_lim && graph[y + 1][x] != '#' {
        let ns = dfs(graph, score + 1, x, y + 1, x_lim, y_lim, score_lim, visited);
        if ns < smallest_score {
            smallest_score = ns;
        }
    }

    smallest_score
}

fn all_connections(
    x: usize,
    y: usize,
    initial_score: u64,
    path_scores: &HashMap<(usize, usize), u64>,
    max_score: u64
) -> u64 {
    let mut count = 0;
    for (end, remaining_time) in path_scores {
        let dist = manhattan_distance(x, y, end.0, end.1);
        if dist > 20 {
            continue;
        }
        let score = initial_score + dist + remaining_time;
        if score <= max_score - 100 {
            count += 1;
        }
    }
    count
}

impl Day for Day20 {
    fn part_1(&mut self) -> u64 {
        let cheat_walls = self.find_cheat_walls();
        println!("Total Cheat Walls: {}", cheat_walls.len());
        let mut visited = HashMap::default();
        let mut count = 0;
        let actual_score = dfs(
            &self.graph,
            0,
            self.start.0,
            self.start.1,
            self.x_lim,
            self.y_lim,
            u64::MAX,
            &mut visited,
        );
        for w in cheat_walls {
            self.graph[w.1][w.0] = '.';
            visited.clear();
            let score = dfs(
                &self.graph,
                0,
                self.start.0,
                self.start.1,
                self.x_lim,
                self.y_lim,
                actual_score - 100,
                &mut visited,
            );
            if score != u64::MAX {
                count += 1;
            }
            self.graph[w.1][w.0] = '#';
        }
        count
    }

    fn part_2(&mut self) -> u64 {
        let mut visited = HashMap::default();
        let actual_score = dfs(
            &self.graph,
            0,
            self.start.0,
            self.start.1,
            self.x_lim,
            self.y_lim,
            u64::MAX,
            &mut visited,
        );
        println!("Actual Score: {}", actual_score);
        let end = find_char(&self.graph, 'E').expect("Cannot find E");
        let mut path = Vec::new();
        self.full_path(self.start.0, self.start.1, ' ', &mut path);
        self.graph[self.start.1][self.start.0] = '.';
        let mut path_score = HashMap::default();
        self.calculate_path_score(actual_score, end.0, end.1, &mut path_score, actual_score);
        
        path.iter().enumerate().map(|(score, p)| {
            all_connections(p.0, p.1, score as u64, &path_score,actual_score)
        }).sum()
        
    }
}
