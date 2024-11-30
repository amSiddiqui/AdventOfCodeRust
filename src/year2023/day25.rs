use std::collections::HashMap;
use std::fs;
use crate::traits::Day;

pub struct Day25 {
    graph: HashMap<String, Vec<String>>
}

impl Day25 {
    pub fn new() -> Self {
        let graph = fs::read_to_string("data/day25")
            .expect("Cannot read file")
            .split('\n')
            .map(|line| {

            });
        todo!()
    }
}

impl Day for Day25 {
    fn part_1(&mut self) -> u64 {
        todo!()
    }

    fn part_2(&mut self) -> u64 {
        todo!()
    }
}
