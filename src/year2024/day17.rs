use std::{collections::HashSet, fs};

use crate::traits::Day;

pub struct Day17 {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
}

pub fn line_to2(line: &str) -> &str {
    line.split(": ").skip(1).next().unwrap()
}

impl Day17 {
    pub fn new() -> Self {
        let data_str = fs::read_to_string("data/year2024/day17").expect("Cannot read data file");
        let mut data = data_str.lines();
        let a_str = data.next().unwrap();
        let a: u64 = line_to2(a_str).parse().unwrap();
        let b_str = data.next().unwrap();
        let b: u64 = line_to2(b_str).parse().unwrap();
        let c_str = data.next().unwrap();
        let c: u64 = line_to2(c_str).parse().unwrap();
        data.next();
        let program_str = data.next().unwrap();
        let program: Vec<u8> = line_to2(program_str)
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        Day17 { a, b, c, program }
    }

    pub fn operation(&mut self, opcode: u8, operand: u8) -> (Option<usize>, Option<usize>) {
        let data = match operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => {
                7
            },
            _ => operand as u64,
        };
        let mut res = None;
        let mut i = None;
        match opcode {
            0 => {
                self.a = self.a / 2_u64.pow(data as u32);
            }
            1 => {
                self.b = self.b ^ (operand as u64);
            }
            2 => {
                self.b = data % 8;
            }
            3 => {
                if self.a > 0 {
                    i = Some(operand as usize);
                }
            }
            4 => {
                self.b = self.b ^ self.c
            }
            5 => {
                res = Some((data % 8) as usize);
            }
            6 => {
                self.b = self.a / 2_u64.pow(data as u32);
            }
            7 => {
                self.c = self.a / 2_u64.pow(data as u32);
            }
            _ => panic!("Unknown opcode"),
        }

        (i, res)
    }
}

impl Day for Day17 {
    fn part_1(&mut self) -> u64 {
        let mut i = 0;
        let mut total = 0;
        println!(
            "a = {}, b = {}, c = {}, program = {:?}",
            self.a, self.b, self.c, self.program
        );
        let mut count = 0;
        let mut set = HashSet::new();
        while i < self.program.len() {
            count += 1;
            let opcode = self.program[i];
            set.insert(opcode);
            let operand = self.program[i + 1];
            let (shift_i, output) = self.operation(opcode, operand);
            if let Some(s_i) = shift_i {
                i = s_i;
                continue;
            } else {
                i += 2;
            }
            if let Some(res) = output {
                print!("{res}");
                total = 10 * total + res;
            }
        }
        println!();
        println!("a = {}, b = {}, c = {}, iters = {count}", self.a, self.b, self.c);
        println!("Unique opcodes = {:?}", set);
        total as u64
    }

    fn part_2(&mut self) -> u64 {
        todo!()
    }
}
