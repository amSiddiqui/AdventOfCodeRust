mod year2023;
mod year2024;
mod traits;

use clap::Parser;
use crate::traits::Day;
use std::{time::Instant, collections::HashMap};

const VALID_YEARS: [u32; 2] = [2023, 2024];


/// Advent of code solutions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Year of challenge
    #[arg(short, long, default_value_t = 2023)]
    year: u32,

    /// Day of the challenge
    #[arg(short, long, default_value_t = 1)]
    day: u32,

    /// Part 1 or 2
    #[arg(short, long, default_value_t = 0)]
    part: u32,
}

fn main() {
    let args = Args::parse();
    if !VALID_YEARS.contains(&args.year) {
        panic!("Please enter a valid year. Valid: {:?}", VALID_YEARS);
    }

    if args.day < 1 || args.day > 25 {
        panic!("Valid days should be between 1 to 25");
    }

    if args.part > 2 {
        panic!("Valid part can only be 1 or 2");
    }
    let mut day_constructors: HashMap<u32, Box<dyn Fn() -> Box<dyn Day>>> = HashMap::new();
    match args.year {
        2023 => {        
            day_constructors.insert(1, Box::new(|| Box::new(year2023::day1::Day1::new()) as Box<dyn Day>));
            day_constructors.insert(2, Box::new(|| Box::new(year2023::day2::Day2::new()) as Box<dyn Day>));
            day_constructors.insert(3, Box::new(|| Box::new(year2023::day3::Day3::new()) as Box<dyn Day>));
            day_constructors.insert(4, Box::new(|| Box::new(year2023::day4::Day4::new()) as Box<dyn Day>));
            day_constructors.insert(5, Box::new(|| Box::new(year2023::day5::Day5::new()) as Box<dyn Day>));
            day_constructors.insert(6, Box::new(|| Box::new(year2023::day6::Day6::new()) as Box<dyn Day>));
            day_constructors.insert(7, Box::new(|| Box::new(year2023::day7::Day7::new()) as Box<dyn Day>));
            day_constructors.insert(8, Box::new(|| Box::new(year2023::day8::Day8::new()) as Box<dyn Day>));
            day_constructors.insert(9, Box::new(|| Box::new(year2023::day9::Day9::new()) as Box<dyn Day>));
            day_constructors.insert(10, Box::new(|| Box::new(year2023::day10::Day10::new()) as Box<dyn Day>));
            day_constructors.insert(11, Box::new(|| Box::new(year2023::day11::Day11::new()) as Box<dyn Day>));
            day_constructors.insert(12, Box::new(|| Box::new(year2023::day12::Day12::new()) as Box<dyn Day>));
            day_constructors.insert(13, Box::new(|| Box::new(year2023::day13::Day13::new()) as Box<dyn Day>));
            day_constructors.insert(14, Box::new(|| Box::new(year2023::day14::Day14::new()) as Box<dyn Day>));
            day_constructors.insert(15, Box::new(|| Box::new(year2023::day15::Day15::new()) as Box<dyn Day>));
            day_constructors.insert(16, Box::new(|| Box::new(year2023::day16::Day16::new()) as Box<dyn Day>));
            day_constructors.insert(17, Box::new(|| Box::new(year2023::day17::Day17::new()) as Box<dyn Day>));
            day_constructors.insert(18, Box::new(|| Box::new(year2023::day18::Day18::new()) as Box<dyn Day>));
            day_constructors.insert(19, Box::new(|| Box::new(year2023::day19::Day19::new()) as Box<dyn Day>));
            day_constructors.insert(20, Box::new(|| Box::new(year2023::day20::Day20::new()) as Box<dyn Day>));
            day_constructors.insert(21, Box::new(|| Box::new(year2023::day21::Day21::new()) as Box<dyn Day>));
            day_constructors.insert(22, Box::new(|| Box::new(year2023::day22::Day22::new()) as Box<dyn Day>));
            day_constructors.insert(23, Box::new(|| Box::new(year2023::day23::Day23::new()) as Box<dyn Day>));
            day_constructors.insert(24, Box::new(|| Box::new(year2023::day24::Day24::new()) as Box<dyn Day>));
        },
        2024 => {
            day_constructors.insert(1, Box::new(|| Box::new(year2024::day1::Day1::new()) as Box<dyn Day>));
        },
        _ => {
            panic!("Solution for year {} is not implemented yet", args.year);
        }
    };
    match day_constructors.get(&args.day) {
        Some(constructor) => {
            let day = constructor();
            run_part(day, args.part);
        },
        _ => panic!("Solution for day {} and year {} is not implemented yet", args.day, args.year),   
    }
}

fn run_part(mut day: Box<dyn Day>, part: u32) {
    if part == 1 || part == 0 {
        let start = Instant::now();
        let s1 = day.part_1();
        let dur = start.elapsed();
        println!("Part 1 solution = {}  ;; Took {:?}", s1, dur);
    }
    if part == 2 || part == 0 {
        let start = Instant::now();
        let s2 = day.part_2();
        let dur = start.elapsed();
        println!("Part 2 solution = {}  ;; Took {:?}", s2, dur);
    }
}
