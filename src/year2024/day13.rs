use std::fs;

use crate::traits::Day;

pub struct Day13 {
    data: Vec<((u64, u64), (u64, u64), (u64, u64))>,
}

fn line_to_point(l: &str) -> (u64, u64) {
    let coord = l.split(':').skip(1).next().unwrap().trim();
    let mut parts = coord.split(", ");
    let x = parts.next().unwrap();
    let y = parts.next().unwrap();
    let start = 2;
    (x[start..].parse().unwrap(), y[start..].parse().unwrap())
}

fn solve_linear_equation(a: (u64, u64), b: (u64, u64), res: (u64, u64)) -> Option<(f64, f64)> {
    let (a0, a1) = a;
    let (b0, b1) = b;
    let (res0, res1) = res;

    let a0 = a0 as f64;
    let a1 = a1 as f64;
    let b0 = b0 as f64;
    let b1 = b1 as f64;
    let res0 = res0 as f64;
    let res1 = res1 as f64;

    let det = a0 * b1 - a1 * b0;
    if det == 0. {
        return None;
    }

    let x = (res0 * b1 - res1 * b0) / det;
    let y = (a0 * res1 - a1 * res0) / det;
    Some((x, y))
}

impl Day13 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2024/day13")
            .expect("cannot read data file")
            .split("\n\n")
            .map(|p| {
                let mut l = p.lines();
                let a = line_to_point(l.next().unwrap());
                let b = line_to_point(l.next().unwrap());
                let res = line_to_point(l.next().unwrap());
                (a, b, res)
            })
            .collect();

        Day13 { data }
    }
}

impl Day for Day13 {
    fn part_1(&mut self) -> u64 {
        let res: f64 = self
            .data
            .iter()
            .map(|(a, b, res)| {
                let (a, b) = solve_linear_equation(*a, *b, *res)
                    .expect(&format!("No solutions for {a:?}, {b:?} {res:?}"));
                if a.fract() == 0.0 && b.fract() == 0.0 {
                    3f64 * a + b
                } else {
                    0.0
                }
            })
            .sum();

        res as u64
    }

    fn part_2(&mut self) -> u64 {
        let res: f64 = self
            .data
            .iter()
            .map(|(a, b, res)| {
                let r = (res.0 + 10000000000000, res.1 + 10000000000000); 
                let (a, b) = solve_linear_equation(*a, *b, r)
                    .expect(&format!("No solutions for {a:?}, {b:?} {res:?}"));
                if a.fract() == 0.0 && b.fract() == 0.0 {
                    3f64 * a + b
                } else {
                    0.0
                }
            })
            .sum();

        res as u64
    }
}
