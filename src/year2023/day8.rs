use std::collections::HashMap;
use std::fs;
use crate::traits::Day;

type Node = [u8; 3];

pub struct Day8 {
    moves: Vec<bool>,
    n: usize,
    map: HashMap<Node, (Node, Node)>
}

impl Day8 {
    fn str_to_u8_array(s: &str) -> Option<Node> {
        let bytes = s.as_bytes();
        if bytes.len() != 3 {
            None
        } else {
            Some([bytes[0], bytes[1], bytes[2]])
        }
    }
    pub fn new() -> Day8 {
        let data = fs::read_to_string("data/day8").unwrap_or_default();
        assert!(!data.is_empty(), "Input file is empty");
        let mut parts = data.split_terminator('\n');
        let moves: Vec<bool> = parts.next().unwrap_or_else(|| panic!("Insufficient data"))
            .chars().map(|x| x == 'R').collect();
        parts.next();
        let mut map = HashMap::new();
        for line in parts {
            let mut line_parts = line.split(" = ");
            let key = Day8::str_to_u8_array(line_parts.next()
                .unwrap_or_else(|| panic!("Line part malformed")))
                .unwrap_or_else(|| panic!("Key does not contain 3 chars"));
            let val = line_parts.next()
                .unwrap_or_else(|| panic!("Line part malformed"));
            let left = Day8::str_to_u8_array(&val[1..4])
                .unwrap_or_else(|| panic!("Map val left does not contain 3 chars"));
            let right = Day8::str_to_u8_array(&val[6..9])
                .unwrap_or_else(|| panic!("Map val right does not contain 3 chars"));
            map.insert(key, (left, right));
        }

        Day8 {
            n: moves.len(),
            moves,
            map
        }
    }



    fn distance_till_z<'a>(&'a self, mut start: &'a Node) -> i32{
        let mut count = 0;
        while start[2] != b'Z' {
            let (l, r) = &self.map[start];
            if self.moves[count % self.n] {
                start = r;
            } else {
                start = l;
            }
            count += 1;
        }
        count as i32
    }

    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    pub fn lcm(arr: Vec<i32>) -> u64 {
        let mut res: u64 = 1;
        for i in arr {
            res = (res * i as u64) / Day8::gcd(res, i as u64);
        }
        res
    }


}

impl Day for Day8 {
    fn part_1(&mut self) -> u64 {
        let mut start = b"AAA";
        let mut count = 0;
        while start != b"ZZZ" {
            let (left, right) = &self.map[start];
            if self.moves[count % self.n] {
                start = right;
            } else {
                start = left;
            }

            count += 1;
        }
        count as u64
    }

    fn part_2(&mut self) -> u64 {
        let distances:Vec<i32> = self.map.keys()
            .filter_map(|x| {
                if x[2] == b'A' {
                    Some(self.distance_till_z(x))
                } else {
                    None
                }
            })
            .collect();
        Day8::lcm(distances)
    }
}
