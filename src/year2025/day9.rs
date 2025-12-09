use ahash::{AHashMap, AHashSet};

use crate::traits::Day;
use std::{fs, time::Instant};

pub struct Day9 {
    data: Vec<(u64, u64)>,
}

impl Day9 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2025/day9")
            .expect("Cannot read data")
            .lines()
            .map(|line| {
                let mut p = line.split(",");
                let x = u64::from_str_radix(p.next().unwrap(), 10).unwrap();
                let y = u64::from_str_radix(p.next().unwrap(), 10).unwrap();
                (x, y)
            })
            .collect();
        Day9 { data }
    }
}


fn within_bound(bounds: &AHashMap<u64, Vec<u64>>, y_bounds: &AHashMap<u64, Vec<u64>>, point: (u64, u64), cache: &mut AHashMap<(u64, u64), bool>) -> bool {
    if let Some(val) = cache.get(&point) {
        return *val;
    }

    let mut crosses = 0;
    if let Some(bound) = bounds.get(&point.0) {
        if point.1 >= bound[0] && point.1 <= bound[1] {
            cache.insert(point, true);
            return true;
        }
    }

    if let Some(bound) = y_bounds.get(&point.1) {
        if point.0 >= bound[0] && point.0 <= bound[1] {
            cache.insert(point, true);
            return true;
        }
    }
    let enable_trace = false;

    let mut x_i = 0;
    if enable_trace {
        println!("Checking for point {point:?}");
    }
    while x_i < point.0 {
        if enable_trace {
            dbg!(&x_i);
        }
        if let Some(bound) = bounds.get(&x_i) {
            if point.1 >= bound[0] && point.1 <= bound[1] {
                // hit a wall
                cache.insert((x_i, point.1), true);

                if let Some(y_bound) = y_bounds.get(&point.1) {
                    if x_i == y_bound[0] {
                        // a corner
                        if enable_trace {
                            println!("Hit a corner");
                        }
                        if point.0 <= y_bound[1] {
                            break;
                        }
                        let next_point_bound = bounds.get(&y_bound[1]).expect("error");
                        if (point.1 > bound[0] && point.1 > next_point_bound[0]) || (point.1 < bound[1] && point.1 < next_point_bound[1]) {
                            // U shaped corner
                            
                        } else {
                            // L shaped so considered a wall
                            crosses += 1;
                        }
                        for x in x_i+1..=y_bound[1] {
                            cache.insert((x, point.1), true);
                        }
                        x_i = y_bound[1] + 1;
                        if enable_trace {
                            println!("{bound:?}  {y_bound:?}  {next_point_bound:?} {crosses}");
                        }
                        
                        continue;
                    } else {
                        // just a wall
                    }
                }
                crosses += 1;
            }
        }
        if enable_trace {
            println!("Crosses: {crosses}");
        }
        x_i += 1;
    }
    
    let res = crosses % 2== 1;
    cache.insert(point, res);
    return res;
}



fn find_in_points(x_bounds: &AHashMap<u64, Vec<u64>>, y_bounds: &AHashMap<u64, Vec<u64>>, x_lim: u64, y_lim: u64) -> AHashSet<(u64, u64)> {
    let mut in_points: AHashSet<(u64, u64)> = AHashSet::new();
    let mut inside = false;
    let mut previous_point: Option<(u64, u64)>  = None;
    for y in 0..y_lim+1 {
        let mut x = 0;
        while x <= x_lim {
            if let Some(prev_point) = previous_point {
                if prev_point.0 == x && prev_point.1 == y {
                    panic!("Loop stuck");
                }
            }
            previous_point = Some((x, y));
            if let Some(x_bound) = x_bounds.get(&x) {
                if y >= x_bound[0] && y <= x_bound[1] {
                    // on wall
                    in_points.insert((x, y));
                    if let Some(y_bound) = y_bounds.get(&y) {
                        if x >= y_bound[0] && x <= y_bound[1] {
                            // on corner
                            let next_point = (y_bound[1], y);
                            let next_x_bound = x_bounds.get(&next_point.0).expect("No point found");
                            if (y > x_bound[0] && y > next_x_bound[0]) || (y < x_bound[1] && y < next_x_bound[1]) {
                            } else {
                                inside = !inside;
                            }
                            for i in x..next_point.0+1 {
                                in_points.insert((i, y));
                            }
                            // println!("On point: ({x}, {y}) {next_point:?} {x_bound:?} {y_bound:?} {next_x_bound:?}");
                            x = next_point.0 + 1;
                            continue;
                        }
                    }
                    inside = !inside;
                    x += 1;
                    continue;
                }
            }
            if inside {
                in_points.insert((x, y));
            }
            x += 1;
        }
    }
    in_points
} 




impl Day for Day9 {
    fn part_1(&mut self) -> u64 {
        let mut area = 0;
        let l = self.data.len();
        for i in 0..l-1 {
            for j in i..l {
                let a = self.data[i];
                let b = self.data[j];
                let h = if a.1 > b.1 { a.1 - b.1 + 1 } else { b.1 - a.1 + 1 };
                let w = if a.0 > b.0 { a.0 - b.0 + 1 } else { b.0 - a.0 + 1 };
                let ar = h * w;
                if ar > area {
                    area = ar;
                }
            }
        }

        area as u64
    }
    fn part_2(&mut self) -> u64 {
        let mut area = 0;
        let l = self.data.len();
        let mut x_bounds: AHashMap<u64, Vec<u64>> = AHashMap::new();
        let mut y_bounds: AHashMap<u64, Vec<u64>> = AHashMap::new();
        for p in self.data.iter() {
            if let Some(bounds) = x_bounds.get_mut(&p.0) {
                if bounds[0] < p.1  {
                    bounds.push(p.1);
                } else {
                    bounds.insert(0, p.1);
                }
            } else {
                x_bounds.insert(p.0, vec![p.1]);
            }
            if let Some(bounds) = y_bounds.get_mut(&p.1) {
                if bounds[0] < p.0  {
                    bounds.push(p.0);
                } else {
                    bounds.insert(0, p.0);
                }
            } else {
                y_bounds.insert(p.1, vec![p.0]);
            }
        }
        let mut x_lim = 0;
        let mut y_lim = 0;
        for p in &self.data {
            if p.0 > x_lim {
                x_lim = p.0;
            }
            if p.1 > y_lim {
                y_lim = p.1;
            }
        }

        // dbg!(&x_bounds);
        // dbg!(&y_bounds);
        // dbg!(x_lim);
        // dbg!(y_lim);
        
        let mut cache: AHashMap<(u64, u64), bool> = AHashMap::new();
        for i in 0..l-1 {
            for j in i+1..l {
                let mut a = self.data[i];
                let mut b = self.data[j];
                if a.0 > b.0 {
                    let temp = a;
                    a = b;
                    b = temp;
                }
                let mut found = false;

                // start marching right to b
                for x in a.0..=b.0 {
                    if !within_bound(&x_bounds, &y_bounds, (x, a.1), &mut cache) {
                        found = true;
                    }
                }
                if found {
                    continue;
                }

                let range = if a.1 < b.1 { a.1..=b.1 } else { b.1..=a.1 };
                for y in range {
                    if !within_bound(&x_bounds, &y_bounds, (a.0, y), &mut cache) {
                        found = true;
                    }
                }
                if found {
                    continue;
                }
                

                let h = if a.1 > b.1 { a.1 - b.1 + 1 } else { b.1 - a.1 + 1 };
                let w = if a.0 > b.0 { a.0 - b.0 + 1 } else { b.0 - a.0 + 1 };
                let ar = h * w;
                // println!("{a:?}, {b:?} ;; {ar}");
                if ar > area {
                    area = ar;
                }
            }
        }

        area as u64
    }
}
