use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

use crate::traits::Day;

pub struct Day12 {
    graph: Vec<Vec<char>>,
}

impl Day12 {
    pub fn new() -> Self {
        let graph = fs::read_to_string("data/year2024/day12")
            .expect("data not found")
            .lines()
            .map(|l| l.chars().collect())
            .collect();

        Day12 { graph }
    }
}

fn calculate_perimeter(graph: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    let ch = graph[y][x];
    let x_lim = graph[0].len() - 1;
    let y_lim = graph.len() - 1;
    let mut perimeter = 0;
    // west
    if x == 0 || graph[y][x - 1] != ch {
        perimeter += 1;
    }

    // east
    if x == x_lim || graph[y][x + 1] != ch {
        perimeter += 1;
    }

    // north
    if y == 0 || graph[y - 1][x] != ch {
        perimeter += 1;
    }

    // south
    if y == y_lim || graph[y + 1][x] != ch {
        perimeter += 1;
    }

    perimeter
}

fn traverse(
    graph: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    area: &mut u64,
    perimeter: &mut u64,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(x, y)) {
        return;
    }
    visited.insert((x, y));
    *perimeter += calculate_perimeter(graph, x, y);
    *area += 1;
    let ch = graph[y][x];

    // west
    if x > 0 && graph[y][x - 1] == ch {
        traverse(graph, x - 1, y, area, perimeter, visited);
    }
    // east
    if x < graph[0].len() - 1 && graph[y][x + 1] == ch {
        traverse(graph, x + 1, y, area, perimeter, visited);
    }
    // north
    if y > 0 && graph[y - 1][x] == ch {
        traverse(graph, x, y - 1, area, perimeter, visited);
    }
    // south
    if y < graph.len() - 1 && graph[y + 1][x] == ch {
        traverse(graph, x, y + 1, area, perimeter, visited);
    }
}

fn traverse_2(
    graph: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    area: &mut u64,
    side: &mut HashSet<(i32, i32, char)>,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(x, y)) {
        return;
    }
    visited.insert((x, y));

    *area += 1;
    let ch = graph[y][x];

    let x_lim = graph[0].len() - 1;
    let y_lim = graph.len() - 1;

    let xi = x as i32;
    let yi = y as i32;
    // west
    if xi == 0 || graph[y][x - 1] != ch {
        side.insert((xi - 1, yi, 'W'));
    }

    // east
    if x == x_lim || graph[y][x + 1] != ch {
        side.insert((xi + 1, yi, 'E'));
    }

    // north
    if y == 0 || graph[y - 1][x] != ch {
        side.insert((xi, yi - 1, 'N'));
    }

    // south
    if y == y_lim || graph[y + 1][x] != ch {
        side.insert((xi, yi + 1, 'S'));
    }

    // west
    if x > 0 && graph[y][x - 1] == ch {
        traverse_2(graph, x - 1, y, area, side, visited);
    }
    // east
    if x < graph[0].len() - 1 && graph[y][x + 1] == ch {
        traverse_2(graph, x + 1, y, area, side, visited);
    }
    // north
    if y > 0 && graph[y - 1][x] == ch {
        traverse_2(graph, x, y - 1, area, side, visited);
    }
    // south
    if y < graph.len() - 1 && graph[y + 1][x] == ch {
        traverse_2(graph, x, y + 1, area, side, visited);
    }
}

impl Day for Day12 {
    fn part_2(&mut self) -> u64 {
        let mut visited_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

        let mut total = 0;
        for (y, line) in self.graph.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                let visited = visited_map.entry(*ch).or_insert(HashSet::new());
                let mut area = 0;
                let mut side = HashSet::new();
                traverse_2(&self.graph, x, y, &mut area, &mut side, visited);
                if area > 0 {
                    let mut side = side.into_iter().collect::<Vec<(i32, i32, char)>>();
                    side.sort_by(|a, b| {
                        a.2.cmp(&b.2).then_with(|| match a.2 {
                            'W' | 'E' => a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)),
                            _ => a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)),
                        })
                    });

                    // count edges
                    let mut count = 0;
                    let mut prev: Option<(i32, i32, char)> = None;
                    side.iter()
                        .filter(|a| a.2 == 'E' || a.2 == 'W')
                        .for_each(|(x, y, dir)| {
                            if let Some(prev) = prev {
                                if prev.2 != *dir || prev.0 != *x || prev.1 != *y - 1 {
                                    count += 1;
                                }
                            } else {
                                count += 1;
                            }
                            prev = Some((*x, *y, *dir));
                        });
                    prev = None;
                    side.iter()
                        .filter(|a| a.2 == 'N' || a.2 == 'S')
                        .for_each(|(x, y, dir)| {
                            if let Some(prev) = prev {
                                if prev.2 != *dir || prev.1 != *y || prev.0 != *x - 1 {
                                    count += 1;
                                }
                            } else {
                                count += 1;
                            }
                            prev = Some((*x, *y, *dir));
                        });

                    total += area * count;
                }
            }
        }
        total
    }

    fn part_1(&mut self) -> u64 {
        let mut visited_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

        let mut total = 0;
        for (y, line) in self.graph.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                let visited = visited_map.entry(*ch).or_insert(HashSet::new());
                let mut area = 0;
                let mut perimeter = 0;
                traverse(&self.graph, x, y, &mut area, &mut perimeter, visited);
                total += area * perimeter;
            }
        }
        total
    }
}
