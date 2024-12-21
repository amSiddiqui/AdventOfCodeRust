use crate::traits::Day;
use ahash::HashMap;
use std::{fs, hash::Hash, u8};

// A -> 0
// > -> 1
// ^ -> 2
// v -> 3
// < -> 4

const RIGHT: u8 = 4;
const UP: u8 = 2;
const DOWN: u8 = 3;
const LEFT: u8 = 1;
const A: u8 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn diff(&self, other: &Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

pub struct Day21 {
    numbers: Vec<String>,
    number_to_coord: HashMap<char, Point>,
    direction_to_coord: HashMap<u8, Point>,
}

impl Day21 {
    pub fn new() -> Self {
        let numbers = fs::read_to_string("data/year2024/day21")
            .expect("Cannot read data file")
            .lines()
            .map(String::from)
            .collect();

        let mut number_to_coord = HashMap::default();
        number_to_coord.insert('7', Point(0, 0));
        number_to_coord.insert('8', Point(1, 0));
        number_to_coord.insert('9', Point(2, 0));
        number_to_coord.insert('4', Point(0, 1));
        number_to_coord.insert('5', Point(1, 1));
        number_to_coord.insert('6', Point(2, 1));
        number_to_coord.insert('1', Point(0, 2));
        number_to_coord.insert('2', Point(1, 2));
        number_to_coord.insert('3', Point(2, 2));
        number_to_coord.insert('0', Point(1, 3));
        number_to_coord.insert('A', Point(2, 3));

        let mut direction_to_coord = HashMap::default();
        direction_to_coord.insert(UP, Point(1, 0));
        direction_to_coord.insert(LEFT, Point(0, 1));
        direction_to_coord.insert(DOWN, Point(1, 1));
        direction_to_coord.insert(RIGHT, Point(2, 1));
        direction_to_coord.insert(A, Point(2, 0));

        Day21 {
            numbers,
            number_to_coord,
            direction_to_coord,
        }
    }

    fn digit_to_sequence(&self, start: &Point, digit: char) -> Vec<u8> {
        let mut sequence = Vec::with_capacity(16);
        let pos = self.number_to_coord.get(&digit).unwrap();
        let diff = start.diff(pos);
        for _ in 0..diff.0.abs() {
            sequence.push(if diff.0 > 0 { LEFT } else { RIGHT });
        }
        for _ in 0..diff.1.abs() {
            sequence.push(if diff.1 > 0 { UP } else { DOWN });
        }

        if (start.1 == 3 && pos.0 == 0) || (start.0 == 0 && pos.1 == 3) {
            d2s_sort(&mut sequence);
        }  else {
            sequence.sort();
        }

        
        sequence.push(A);
        sequence
    }

    fn sequence_to_sequence(
        &self,
        start: &Point,
        seq: u8,
        cache: &mut HashMap<(Point, u8), Vec<u8>>,
    ) -> Vec<u8> {
        if let Some(cached_seq) = cache.get(&(*start, seq)) {
            return cached_seq.clone();
        }
        let start = start;
        let pos = self.direction_to_coord.get(&seq).unwrap();
        let mut sequence = Vec::new();
        
        let diff = start.diff(&pos);
        for _ in 0..diff.0.abs() {
            sequence.push(if diff.0 > 0 { LEFT } else { RIGHT });
        }
        for _ in 0..diff.1.abs() {
            sequence.push(if diff.1 > 0 { UP } else { DOWN });
        }

        if (start.1 == 1 && start.0 == 0 && pos.1 == 0) || (start.1 == 0 && pos.0 == 0 && pos.1 == 1) {
            s2s_sort(&mut sequence);
        } else {
            sequence.sort();
        }
        sequence.push(A);
        cache.insert((*start, seq), sequence.clone());
        sequence
    }

    fn dfs_sequence_length(
        &self,
        sequence: &[u8],
        depth: usize,
        direction_map: &HashMap<u8, Point>,
        seq_cache: &mut HashMap<(Point, u8), Vec<u8>>,
        cache: &mut HashMap<(Vec<u8>, usize), usize>
    ) -> usize {
        if depth == 0 {
            return sequence.len();
        }
        
        if let Some(cached_length) = cache.get(&(sequence.to_vec(), depth)) {
            return *cached_length;
        }

        let mut d_start = direction_map[&A];
        let mut sum = 0;
        for &c in sequence {
            let seq = self.sequence_to_sequence(&d_start, c, seq_cache);
            sum += self.dfs_sequence_length(&seq, depth - 1, direction_map, seq_cache, cache);
            d_start = direction_map[&c];
        }
        cache.insert((sequence.to_vec(), depth), sum);
        sum
    }
}

fn d2s_sort(seq: &mut Vec<u8>) {
    fn custom_rank(n: u8) -> u8 {
        match n {
            UP => 0,
            RIGHT => 1,
            DOWN => 2,
            LEFT => 3,
            _ => 4,
        }
    }

    seq.sort_by_key(|&x| custom_rank(x));
}

fn s2s_sort(seq: &mut Vec<u8>) {
    fn custom_rank(n: u8) -> u8 {
        match n {
            UP => 1,
            RIGHT => 0,
            DOWN => 2,
            LEFT => 3,
            _ => 4,
        }
    }   
   seq.sort_by_key(|&x| custom_rank(x));
}


fn string_number_to_num(number: &str) -> u64 {
    let number = number.replace('A', "");
    number.parse().unwrap()
}

impl Day for Day21 {
    fn part_1(&mut self) -> u64 {
        
        let u8_to_char = HashMap::from_iter(vec![
            (A, 'A'),
            (RIGHT, '>'),
            (UP, '^'),
            (DOWN, 'v'),
            (LEFT, '<'),
        ]);

        let number_map = &self.number_to_coord;
        let direction_map = &self.direction_to_coord;
        self.numbers
            .iter()
            .map(|sample| {
                let mut start = number_map[&'A'];
                let mut cache = HashMap::default();
                let mut sequence = Vec::new();
                for digit in sample.chars() {
                    sequence.extend(self.digit_to_sequence(&start, digit));
                    start = number_map[&digit];
                }
                // print sequence
                print!("Sequence: ");
                for seq in &sequence {
                    print!("{}", u8_to_char[seq]);
                }
                println!();
                let mut s2s = Vec::new();
                let mut s_start = direction_map[&A];
                for seq in &sequence {
                    s2s.extend(self.sequence_to_sequence(&s_start, *seq, &mut cache));
                    s_start = direction_map[seq];
                }
                print!("S2S: ");
                for seq in &s2s {
                    print!("{}", u8_to_char[seq]);
                }
                println!();
                let mut final_seq = Vec::new();
                let mut f_start = direction_map[&A];
                for seq in &s2s {
                    final_seq.extend(self.sequence_to_sequence(&f_start, *seq, &mut cache));
                    f_start = direction_map[seq];
                }
                print!("Final: ");
                let num = string_number_to_num(sample);
                for seq in &final_seq {
                    print!("{}", u8_to_char[seq]);
                }
                println!(";; Len: {}, num: {}", final_seq.len(), num);
                println!();
                final_seq.len() as u64 * num
            })
            .sum()
    }

    fn part_2(&mut self) -> u64 {
        let number_map = &self.number_to_coord;
        let direction_map = &self.direction_to_coord;
        let mut seq_cache = HashMap::default();
        let mut cache = HashMap::default();
        self.numbers
            .iter()
            .map(|sample| {
                let mut start = number_map[&'A'];
                let mut sequence = Vec::new();
                for digit in sample.chars() {
                    sequence.extend(self.digit_to_sequence(&start, digit));
                    start = number_map[&digit];
                }
                let length = self.dfs_sequence_length(
                    &sequence,
                    25,
                    direction_map,
                    &mut seq_cache,
                    &mut cache,
                );
                let num = string_number_to_num(sample);
                length as u64 * num
            })
            .sum()
    }
}
