use std::{collections::{HashMap, HashSet}, fs, u64};

use crate::traits::Day;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum DIR {
    EAST,
    WEST,
    NORTH,
    SOUTH,
}

pub struct Day16 {
    grid: Vec<Vec<char>>,
}

impl Day16 {
    pub fn new() -> Self {
        let grid = fs::read_to_string("data/year2024/day16")
            .expect("data file not found")
            .lines()
            .map(|l| l.chars().collect())
            .collect();

        Day16 { grid }
    }

    pub fn find_char(&self, ch: &char) -> Option<(usize, usize)> {
        for (y, line) in self.grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if c == ch {
                    return Some((x, y));
                }
            }
        }

        None
    }
}

pub fn dfs(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dir: DIR,
    score: u64,
    visited: &mut HashMap<(usize, usize, DIR), u64>,
) -> u64 {
    if let Some(sc) = visited.get(&(x, y, dir)) {
        if *sc <= score {
            return u64::MAX;
        }
    }
    visited.insert((x, y, dir), score);

    if grid[y][x] == 'E' {
        return score;
    }
    let mut scores = vec![];
    // west
    if x > 0 && grid[y][x - 1] != '#' && dir != DIR::EAST {
        let pen = match dir {
            DIR::WEST => 1,
            _ => 1001,
        };
        scores.push(dfs(grid, x - 1, y, DIR::WEST, score + pen, visited));
    }

    // east
    if x < grid[0].len() - 1 && grid[y][x + 1] != '#' && dir != DIR::WEST {
        let pen = match dir {
            DIR::EAST => 1,
            _ => 1001,
        };
        scores.push(dfs(grid, x + 1, y, DIR::EAST, score + pen, visited));
    }

    // north
    if y > 0 && grid[y - 1][x] != '#' && dir != DIR::SOUTH {
        let pen = match dir {
            DIR::NORTH => 1,
            _ => 1001,
        };
        scores.push(dfs(grid, x, y - 1, DIR::NORTH, score + pen, visited));
    }

    // south
    if y < grid.len() - 1 && grid[y + 1][x] != '#' && dir != DIR::NORTH {
        let pen = match dir {
            DIR::SOUTH => 1,
            _ => 1001,
        };
        scores.push(dfs(grid, x, y + 1, DIR::SOUTH, score + pen, visited));
    }

    *scores.iter().min().unwrap_or(&u64::MAX)
}

pub fn dfs2(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dir: DIR,
    score: u64,
    visited: &mut HashMap<(usize, usize, DIR), u64>,
    path: &mut Vec<(usize, usize)>,
) -> (u64, Vec<Vec<(usize, usize)>>) {
    if let Some(sc) = visited.get(&(x, y, dir)) {
        if *sc < score {
            return (u64::MAX, vec![]);
        }
    }
    visited.insert((x, y, dir), score);
    path.push((x, y));

    if grid[y][x] == 'E' {
        let res = (score, vec![path.clone()]);
        path.pop();
        return res;
    }

    let mut best = u64::MAX;
    let mut paths = vec![];

    if x > 0 && grid[y][x - 1] != '#' && dir != DIR::EAST {
        let pen = match dir {
            DIR::WEST => 1,
            _ => 1001,
        };
        let (child_score, child_paths) =
            dfs2(grid, x - 1, y, DIR::WEST, score + pen, visited, path);
        if child_score < best {
            best = child_score;
            paths = child_paths;
        } else if child_score == best {
            paths.extend(child_paths);
        }
    }

    if x < grid[0].len() - 1 && grid[y][x + 1] != '#' && dir != DIR::WEST {
        let pen = match dir {
            DIR::EAST => 1,
            _ => 1001,
        };
        let (child_score, child_paths) =
            dfs2(grid, x + 1, y, DIR::EAST, score + pen, visited, path);
        if child_score < best {
            best = child_score;
            paths = child_paths;
        } else if child_score == best {
            paths.extend(child_paths);
        }
    }

    if y > 0 && grid[y - 1][x] != '#' && dir != DIR::SOUTH {
        let pen = match dir {
            DIR::NORTH => 1,
            _ => 1001,
        };
        let (child_score, child_paths) =
            dfs2(grid, x, y - 1, DIR::NORTH, score + pen, visited, path);
        if child_score < best {
            best = child_score;
            paths = child_paths;
        } else if child_score == best {
            paths.extend(child_paths);
        }
    }

    if y < grid.len() - 1 && grid[y + 1][x] != '#' && dir != DIR::NORTH {
        let pen = match dir {
            DIR::SOUTH => 1,
            _ => 1001,
        };
        let (child_score, child_paths) =
            dfs2(grid, x, y + 1, DIR::SOUTH, score + pen, visited, path);
        if child_score < best {
            best = child_score;
            paths = child_paths;
        } else if child_score == best {
            paths.extend(child_paths);
        }
    }

    path.pop();
    (best, paths)
}

impl Day for Day16 {
    fn part_1(&mut self) -> u64 {
        let (x, y) = self.find_char(&'S').expect("Start not found");
        let mut visited = HashMap::new();
        dfs(&self.grid, x, y, DIR::EAST, 0, &mut visited)
    }

    fn part_2(&mut self) -> u64 {
        let (x, y) = self.find_char(&'S').expect("Start not found");
        let mut visited = HashMap::new();
        let mut path = Vec::new();
        let (_, all_paths) =
            dfs2(&self.grid, x, y, DIR::EAST, 0, &mut visited, &mut path);

        let mut path_set = HashSet::new();
        for path in all_paths.iter() {
            for p in path {
                path_set.insert(*p);
            }
        }

        path_set.len() as u64
    }
}
