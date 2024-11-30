use crate::traits::Day;

pub struct Day1 {
    lines: Vec<String>
}


impl Day1 {
    pub fn new() -> Self {
        Day1 {
            lines: vec![]
        }
    }
}

impl Day for Day1 {
    fn part_1(&mut self) -> u64 {
        self.lines.len() as u64
    }

    fn part_2(&mut self) -> u64 {
        self.lines.len() as u64
    }
}
