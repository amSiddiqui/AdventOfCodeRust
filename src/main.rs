mod year2023;
mod traits;

use clap::Parser;
use crate::traits::Day;
use std::{time::Instant, collections::HashMap};
use crate::year2023::{day1, day10, day11, day12, day13, day2, day3, day4, day5, day9, day14, day6, day7, day8, day15, day16, day17, day18};

const VALID_YEARS: [u32; 1] = [2023];


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

    if args.year == 2023 {
        let mut day_constructors: HashMap<u32, Box<dyn Fn() -> Box<dyn Day>>> = HashMap::new();
        day_constructors.insert(1, Box::new(|| Box::new(day1::Day1::new()) as Box<dyn Day>));
        day_constructors.insert(2, Box::new(|| Box::new(day2::Day2::new()) as Box<dyn Day>));
        day_constructors.insert(3, Box::new(|| Box::new(day3::Day3::new()) as Box<dyn Day>));
        day_constructors.insert(4, Box::new(|| Box::new(day4::Day4::new()) as Box<dyn Day>));
        day_constructors.insert(5, Box::new(|| Box::new(day5::Day5::new()) as Box<dyn Day>));
        day_constructors.insert(6, Box::new(|| Box::new(day6::Day6::new()) as Box<dyn Day>));
        day_constructors.insert(7, Box::new(|| Box::new(day7::Day7::new()) as Box<dyn Day>));
        day_constructors.insert(8, Box::new(|| Box::new(day8::Day8::new()) as Box<dyn Day>));
        day_constructors.insert(9, Box::new(|| Box::new(day9::Day9::new()) as Box<dyn Day>));
        day_constructors.insert(10, Box::new(|| Box::new(day10::Day10::new()) as Box<dyn Day>));
        day_constructors.insert(11, Box::new(|| Box::new(day11::Day11::new()) as Box<dyn Day>));
        day_constructors.insert(12, Box::new(|| Box::new(day12::Day12::new()) as Box<dyn Day>));
        day_constructors.insert(13, Box::new(|| Box::new(day13::Day13::new()) as Box<dyn Day>));
        day_constructors.insert(14, Box::new(|| Box::new(day14::Day14::new()) as Box<dyn Day>));
        day_constructors.insert(15, Box::new(|| Box::new(day15::Day15::new()) as Box<dyn Day>));
        day_constructors.insert(16, Box::new(|| Box::new(day16::Day16::new()) as Box<dyn Day>));
        day_constructors.insert(17, Box::new(|| Box::new(day17::Day17::new()) as Box<dyn Day>));
        day_constructors.insert(18, Box::new(|| Box::new(day18::Day18::new()) as Box<dyn Day>));
        if let Some(constructor) = day_constructors.get(&args.day) {
            let day = constructor();
            run_part(day, args.part);
        } else {
            println!("Solution for day {} and year {} is not implemented yet", args.day, args.year);
        }        
    } else {
        println!("Solution for year {} is not implemented yet", args.year);
    }
}

fn run_part(day: Box<dyn Day>, part: u32) {
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
