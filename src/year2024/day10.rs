use std::{collections::HashSet, fs};

use crate::traits::Day;

#[derive(Eq, PartialEq)]
enum Dir {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

pub struct Day10 {
    pub lines: Vec<Vec<u8>>,
    pub starting: Vec<(usize, usize)>,
}

impl Day10 {
    pub fn new() -> Self {
        let lines: Vec<Vec<u8>> = fs::read_to_string("data/year2024/day10")
            .expect("Cannot read data file")
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();

        let mut starting = vec![];

        lines.iter().enumerate().for_each(|(y, l)| {
            l.iter().enumerate().for_each(|(x, c)| {
                if *c == 0 {
                    starting.push((x, y));
                }
            });
        });

        Day10 { lines, starting }
    }
}

fn graph_traverse(graph: &Vec<Vec<u8>>, x: usize, y: usize, expected: u8, visited: &mut HashSet<(usize, usize)>) -> u64 {
    if visited.contains(&(x, y)) {
        return 0;
    }

    visited.insert((x, y));

    if graph[y][x] == 9 {
        return 1;
    }

    let mut score = 0;
    
    // west
    if x > 0 && graph[y][x-1] == expected {
        score += graph_traverse(graph, x-1, y, expected+1, visited);
    }

    // east
    if x < graph[0].len() - 1 && graph[y][x+1] == expected {
        score += graph_traverse(graph, x+1, y, expected+1, visited);
    }

    // north
    if y > 0 && graph[y-1][x] == expected {
        score += graph_traverse(graph, x, y-1, expected+1, visited);
    }

    // south
    if y < graph.len() - 1 && graph[y+1][x] == expected {
        score += graph_traverse(graph, x, y+1, expected+1, visited);
    }

    score
}

fn graph_distinct_path(graph: &Vec<Vec<u8>>, x: usize, y: usize, expected: u8, dir: Dir) -> u64 {
    if graph[y][x] == 9 {
        return 1;
    }

    let mut score = 0;
    
    // west
    if dir != Dir::WEST && x > 0 && graph[y][x-1] == expected {
        score += graph_distinct_path(graph, x-1, y, expected+1, Dir::EAST);
    }

    // east
    if dir != Dir::EAST && x < graph[0].len() - 1 && graph[y][x+1] == expected {
        score += graph_distinct_path(graph, x+1, y, expected+1, Dir::WEST);
    }

    // north 
    if dir != Dir::NORTH && y > 0 && graph[y-1][x] == expected {
        score += graph_distinct_path(graph, x, y-1, expected+1, Dir::SOUTH);
    }

    // south
    if dir != Dir::SOUTH && y < graph.len() - 1 && graph[y+1][x] == expected {
        score += graph_distinct_path(graph, x, y+1, expected+1, Dir::NORTH);
    }


    score
}


impl Day for Day10 {
    fn part_1(&mut self) -> u64 {
        let mut total = 0;
        for starts in &self.starting {
            let mut visited = HashSet::new();
            let res = graph_traverse(&self.lines, starts.0, starts.1, 1, &mut visited);
            total += res;
        }
        total
    }

    fn part_2(&mut self) -> u64 {
        let mut total = 0;
        let graph = &self.lines;
        for (x, y) in &self.starting {
            let mut score = 0;
            let expected = 1;
            // west
            if *x > 0 && graph[*y][x-1] == expected {
                score += graph_distinct_path(graph, x-1, *y, expected+1, Dir::EAST);
            }

            // east
            if *x < graph[0].len() - 1 && graph[*y][x+1] == expected {
                score += graph_distinct_path(graph, x+1, *y, expected+1, Dir::WEST);
            }

            // north 
            if *y > 0 && graph[y-1][*x] == expected {
                score += graph_distinct_path(graph, *x, y-1, expected+1, Dir::SOUTH);
            }

            // south
            if *y < graph.len() - 1 && graph[y+1][*x] == expected {
                score += graph_distinct_path(graph, *x, y+1, expected+1, Dir::NORTH);
            }

            total += score;
        }
        total
        
    }
}
