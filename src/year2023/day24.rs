use std::fs;
use crate::traits::Day;


const LOW: f64 = 200000000000000.;
const HIGH: f64 = 400000000000000.;

pub struct Day24 {
    lines: Vec<([f64; 3], [f64; 3])>
}

impl Day24 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day24")
            .expect("Cannot read file")
            .split('\n')
            .map(|line| {
                let (p1, dp2) = line.split_once('@').expect("Line malformed");
                let p1_nums = Day24::str_to_vec3(p1);
                let dp2_nums = Day24::str_to_vec3(dp2);
                ([p1_nums[0], p1_nums[1], p1_nums[2]], [dp2_nums[0], dp2_nums[1], dp2_nums[2]])
            }).collect::<Vec<_>>();

        Day24 {
            lines
        }
    }

    fn str_to_vec3(point: &str) -> Vec<f64> {
        point.trim().split(',')
            .map(|x| x.trim().parse::<f64>()
                .unwrap_or_else(|err| panic!("Cannot parse {x};; Error: {err}"))
            ).collect::<Vec<_>>()
    }

    pub fn convert_to_2d_lines(&self) -> Vec<(f64, f64)> {
    //     converting 2 points to y = mx + c
        self.lines.iter().map(|(p1_nums, dp2_nums)| {
            let m = dp2_nums[1] / dp2_nums[0];
            let c = p1_nums[1] - m * p1_nums[0];
            (m, c)
        }).collect::<Vec<_>>()
    }

    pub fn intersection(l1: &(f64, f64), l2: &(f64, f64)) -> (f64, f64) {
    //     line is in form y = mx + c
        let (m1, c1) = l1;
        let (m2, c2) = l2;
        let x = (c2 - c1) / (m1 - m2);
        let y = m1* x + c1;
        (x, y)
    }

    pub fn magnitude(v: &(f64, f64)) -> f64 {
        let (x, y) = v;
        (x * x + y * y).sqrt()
    }

    pub fn cosine_2_vec(v1: &(f64, f64), v2: &(f64, f64)) -> f64 {
        let (x1, y1) = v1;
        let (x2, y2) = v2;
        let dot = x1 * x2 + y1 * y2;
        dot / (Day24::magnitude(v1) * Day24::magnitude(v2))
    }

    pub fn check_point_is_future(line: &([f64; 3], [f64; 3]), point: &(f64, f64)) -> bool {
        let v1 = (line.1[0], line.1[1]);
        let v2 = (point.0 - line.0[0], point.1 - line.0[1]);
        let cos = Day24::cosine_2_vec(&v1, &v2);
        cos.round() == 1.
    }

}

impl Day for Day24 {
    fn part_1(&mut self) -> u64 {
        let lines = self.convert_to_2d_lines();
        let mut count = 0;
        for i in 0..lines.len() - 1 {
            for j in i+1..lines.len() {
                let (x, y) = Day24::intersection(&lines[i], &lines[j]);
                let is_future_of_i = Day24::check_point_is_future(&self.lines[i], &(x, y));
                let is_future_of_j = Day24::check_point_is_future(&self.lines[j], &(x, y));
                if x >= LOW && y >= LOW && x <= HIGH && y <= HIGH
                    && is_future_of_i
                    && is_future_of_j
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn part_2(&mut self) -> u64 {
        todo!()
    }
}
