use std::fs;
use crate::traits::Day;
use rayon::prelude::*;

type SeedMap = (u64, u64, u64);

pub struct Day5 {
    seeds: Vec<u64>,
    maps: Vec<Vec<SeedMap>>
}

impl Day5 {
    fn parse_line(line: &str) -> SeedMap {
        let mut parts = line.split_whitespace()
            .map(|x| x.parse::<u64>().unwrap_or_else(|err| panic!("Error parsing line {};; Error {}", x, err)));
        let a = parts.next().unwrap_or_else(|| panic!("Dest map not present"));
        let b = parts.next().unwrap_or_else(|| panic!("Source map not present"));
        let c = parts.next().unwrap_or_else(|| panic!("Map range not present"));
        (a, b, c)
    }

    pub fn new() -> Self {
        let data = fs::read_to_string("data/day5")
            .unwrap_or_else(|err| panic!("Error reading file {}", err));
        let mut parts = data.split("\n\n");
        let seeds: Vec<u64> = parts.next()
            .unwrap_or_else(|| panic!("Seed data not available"))
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u64>()
                .unwrap_or_else(|err| panic!("Cannot parse input {};; Err {}", x, err)))
            .collect();

        let maps: Vec<_> = parts.map(|x| x.split('\n')
            .skip(1)
            .map(Day5::parse_line)
            .collect()
        ).collect();

        Day5 {
            seeds,
            maps
        }
    }

    fn apply_map(&self, mut seed: u64) -> u64 {
        for m in &self.maps {
            for (dest, source, range) in m {
                if *source <= seed && seed < source + range {
                    seed = seed - source + dest;
                    break;
                }
            }
        }
        seed
    }

    fn apply_range_brute_force(&self, start: u64, range: u64) -> u64{
        let mut min = u64::MAX;
        for i in 0..range {
            let r = self.apply_map(i + start);
            if r < min {
                min = r;
            }
        }
        min
    }

}

impl Day for Day5 {
    fn part_1(&self) -> u64 {
        let res = self.seeds.par_iter().map(|x| self.apply_map(*x)).min();
        res.unwrap_or(0)
    }

    fn part_2(&self) -> u64 {
        let seeds:Vec<_> = self.seeds.chunks(2)
            .map(|chunk| (chunk[0], chunk[1]))
            .collect();

        let res = seeds.par_iter()
            .map(|(l, r)| self.apply_range_brute_force(*l, *r))
            .min();

        res.unwrap_or(0)
    }
}

