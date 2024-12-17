use std::{collections::HashSet, fs};

use crate::traits::Day;

pub struct Day17 {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
}

pub fn line_to2(line: &str) -> &str {
    line.split(": ").nth(1).unwrap()
}

impl Day17 {
    pub fn new() -> Self {
        let data_str = fs::read_to_string("data/year2024/day17").expect("Cannot read data file");
        let mut data = data_str.lines();
        let a: u64 = line_to2(data.next().unwrap()).parse().unwrap();
        let b: u64 = line_to2(data.next().unwrap()).parse().unwrap();
        let c: u64 = line_to2(data.next().unwrap()).parse().unwrap();
        data.next();
        let program: Vec<u64> = line_to2(data.next().unwrap())
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        Day17 { a, b, c, program }
    }

    #[inline(always)]
    pub fn operation(&mut self, opcode: u64, operand: u64) -> (Option<usize>, Option<u64>) {
        let combo = match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => 7,
            _ => panic!("Unknown operand"),
        };

        match opcode {
            0 => {
                self.a >>= combo as u32;
                (None, None)
            }
            1 => {
                self.b ^= operand;
                (None, None)
            }
            2 => {
                self.b = combo & 7;
                (None, None)
            }
            3 => {
                if self.a != 0 {
                    (Some(operand as usize), None)
                } else {
                    (None, None)
                }
            }
            4 => {
                self.b ^= self.c;
                (None, None)
            }
            5 => {
                (None, Some(combo & 7))
            }
            6 => {
                self.b = self.a >> combo as u32;
                (None, None)
            }
            7 => {
                self.c = self.a >> combo as u32;
                (None, None)
            }
            _ => panic!("Unknown opcode"),
        }
    }
}

impl Day for Day17 {
    fn part_1(&mut self) -> u64 {
        let mut i = 0;
        let mut solution = Vec::new();
        let program_len = self.program.len();

        while i < program_len {
            let opcode = self.program[i];
            let operand = self.program[i + 1];
            if operand == 7 && opcode != 4 {
                panic!("Invalid operand");
            }
            let (shift_i, output) = self.operation(opcode, operand);
            if let Some(s_i) = shift_i {
                i = s_i;
                continue;
            } else {
                i += 2;
            }
            if let Some(res) = output {
                solution.push(res.to_string());
            }
        }
        let output = solution.join(",");
        println!("Solution = {}", output);
        0
    }
    fn part_2(&mut self) -> u64 {
        let program_len = self.program.len();
        let mut a = 8u64.pow(program_len as u32) / 8;
        let limit = a * 8;
        loop {
            self.a = a;
            self.b = 0;
            self.c = 0;
            let mut i = 0;
            let mut states = HashSet::new();
            let mut output = vec![];


            while i < program_len {
                if states.contains(&(self.a, self.b, self.c, i)) {
                    break;
                }
                states.insert((self.a, self.b, self.c, i));
                let opcode = self.program[i];
                let operand = self.program[i + 1];
                if operand == 7 && opcode != 4 {
                    panic!("Invalid operand");
                }
                let (shift_i, out) = self.operation(opcode, operand);
                
                if let Some(s_i) = shift_i {
                    i = s_i;
                    continue;
                } else {
                    i += 2;
                }

                if let Some(res) = out {
                    output.push(res);
                    if output == self.program {
                        return a;
                    }
                }
            }

            // digits matched from 
            let matches = output.iter().rev().zip(self.program.iter().rev()).take_while(|(x, y)| x == y).count();
            a += 8u64.pow((program_len - matches - 1) as u32);
            if a > limit {
                panic!("limit reached");
            }
            let out_str = output.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(",");
            println!("a = {a} ;; Out = {out_str}");
            
        }
    }
}