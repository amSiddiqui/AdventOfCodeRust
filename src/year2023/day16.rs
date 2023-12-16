use std::collections::HashSet;
use std::fs;
use rayon::prelude::*;

use crate::traits::Day;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum Direction {
    South,
    North,
    East,
    West
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        match self {
            Direction::East | Direction::West => true,
            _ => false
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Hash, Clone, Eq, PartialEq)]
struct Beam {
    direction: Direction,
    point: Point
}

impl Point {
    fn is_out_of_bound(&self, bound: &Point) -> bool {
        self.x < 0 || self. y < 0 || self.x >= bound.x || self.y >= bound.y
    }
}


pub struct Day16 {
    lines: Vec<Vec<char>>,
    bound: Point
}

impl Day16 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day16")
            .expect("Cannot read file")
            .split('\n')
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let bound = Point { x: lines.len() as i32, y: lines[0].len() as i32 };
        Day16{
            lines,
            bound
        }
    }

    fn traverse(&self, mut beam: Beam, visited: &mut HashSet<Beam>) {
        while !visited.contains(&beam) && !beam.point.is_out_of_bound(&self.bound) {
            visited.insert(beam.clone());
            let tile = self.lines[beam.point.x as usize][beam.point.y as usize];

            if (tile == '|' && !beam.direction.is_horizontal()) ||
                (tile == '-' && beam.direction.is_horizontal()) ||
                tile == '.' {
                match beam.direction {
                    Direction::South => {
                        let next_point =  Point { x: beam.point.x+1, y: beam.point.y };
                        beam.point = next_point;
                    }
                    Direction::North => {
                        let next_point =  Point { x: beam.point.x-1, y: beam.point.y };
                        beam.point = next_point;
                    }
                    Direction::East => {
                        let next_point =  Point { x: beam.point.x, y: beam.point.y+1 };
                        beam.point = next_point;
                    }
                    Direction::West => {
                        let next_point =  Point { x: beam.point.x, y: beam.point.y-1 };
                        beam.point = next_point;
                    }
                }
                continue;
            }

            if tile == '/' {
                match beam.direction {
                    Direction::South => {
                        let next_point =  Point { x: beam.point.x, y: beam.point.y-1 };
                        beam.point = next_point;
                        beam.direction = Direction::West;
                    }
                    Direction::North => {
                        let next_point =  Point { x: beam.point.x, y: beam.point.y+1 };
                        beam.point = next_point;
                        beam.direction = Direction::East;
                    }
                    Direction::East => {
                        let next_point =  Point { x: beam.point.x-1, y: beam.point.y };
                        beam.point = next_point;
                        beam.direction = Direction::North;
                    }
                    Direction::West => {
                        let next_point =  Point { x: beam.point.x+1, y: beam.point.y };
                        beam.point = next_point;
                        beam.direction = Direction::South;
                    }
                }
                continue;
            }

            if tile == '\\' {
                match beam.direction {
                    Direction::South => {
                        let next_point =  Point { x: beam.point.x, y: beam.point.y+1 };
                        beam.point = next_point;
                        beam.direction = Direction::East;
                    }
                    Direction::North => {
                        let next_point =  Point { x: beam.point.x, y: beam.point.y-1 };
                        beam.point = next_point;
                        beam.direction = Direction::West;
                    }
                    Direction::East => {
                        let next_point =  Point { x: beam.point.x+1, y: beam.point.y };
                        beam.point = next_point;
                        beam.direction = Direction::South;
                    }
                    Direction::West => {
                        let next_point =  Point { x: beam.point.x-1, y: beam.point.y };
                        beam.point = next_point;
                        beam.direction = Direction::North;
                    }
                }
                continue;
            }

            if tile == '|' {
                let beam_1 = Beam {
                    direction: Direction::North,
                    point: Point {
                        x: beam.point.x - 1,
                        y: beam.point.y
                    }
                };
                let beam_2 = Beam {
                    direction: Direction::South,
                    point: Point {
                        x: beam.point.x + 1,
                        y: beam.point.y
                    }
                };
                self.traverse(beam_1, visited);
                self.traverse(beam_2, visited);
                return;
            }

            if tile == '-' {
                let beam_1 = Beam {
                    direction: Direction::East,
                    point: Point {
                        x: beam.point.x,
                        y: beam.point.y + 1
                    }
                };
                let beam_2 = Beam {
                    direction: Direction::West,
                    point: Point {
                        x: beam.point.x,
                        y: beam.point.y - 1
                    }
                };
                self.traverse(beam_1, visited);
                self.traverse(beam_2, visited);
                return;
            }
        }
    }

    fn start_travel(&self, beam: Beam) -> u64 {
        let mut visited:HashSet<Beam> = HashSet::new();
        self.traverse(beam, &mut visited);
        visited
            .iter()
            .map(|beam| beam.point.clone())
            .collect::<HashSet<_>>()
            .len() as u64
    }
}

impl Day for Day16 {
    fn part_1(&self) -> u64 {
        self.start_travel(Beam {
            direction: Direction::East,
            point: Point { x: 0, y: 0 }
        })
    }

    fn part_2(&self) -> u64 {
        let mut beams: Vec<Beam> = vec![];
        for i in 0..self.bound.y {
            beams.push(Beam {
                direction: Direction::South,
                point: Point{ x: 0, y: i }
            });
        }

        for i in 0..self.bound.x {
            beams.push(Beam {
                direction: Direction::East,
                point: Point{ x: i, y: 0 }
            });
        }

        for i in 0..self.bound.x {
            beams.push(Beam {
                direction: Direction::West,
                point: Point{ x: i, y: self.bound.y - 1 }
            });
        }

        for i in 0..self.bound.y {
            beams.push(Beam {
                direction: Direction::North,
                point: Point{ x: self.bound.x - 1, y: i }
            });
        }


        beams
            .into_par_iter()
            .map(|x| self.start_travel(x))
            .max()
            .expect("No beams found")
    }
}


