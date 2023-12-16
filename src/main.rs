mod year2023;
mod traits;

use clap::Parser;
use year2023::{day8, day7, day6};
use crate::traits::Day;
use std::time::Instant;
use crate::year2023::{day1, day10, day11, day12, day13, day2, day3, day4, day5, day9};

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
        if args.day == 1 {
            let day = day1::Day1::new();
            run_part(day, args.part);
        } else if args.day == 2 {
            let day = day2::Day2::new();
            run_part(day, args.part);
        } else if args.day == 3 {
            let day = day3::Day3::new();
            run_part(day, args.part);
        } else if args.day == 4 {
            let day = day4::Day4::new();
            run_part(day, args.part);
        } else if args.day == 5 {
            let day = day5::Day5::new();
            if args.part == 2 {
                println!("Go grab a coffee. We are going to be here for a while. 😅");
                println!("Your CPU goes brrrrr....");
            }
            run_part(day, args.part);
        } else if args.day == 6 {
            let day = day6::Day6::new();
            run_part(day, args.part);
        } else if args.day == 7 {
            let day = day7::Day7::new();
            run_part(day, args.part);
        } else if args.day == 8 {
            let day = day8::Day8::new();
            run_part(day, args.part);
        } else if args.day == 9 {
            let day = day9::Day9::new();
            run_part(day, args.part);
        } else if args.day == 10 {
            let day = day10::Day10::new();
            run_part(day, args.part);
        } else if args.day == 11 {
            let day = day11::Day11::new();
            run_part(day, args.part);
        } else if args.day == 12 {
            let day = day12::Day12::new();
            run_part(day, args.part);
        } else if args.day == 13 {
            let day = day13::Day13::new();
            run_part(day, args.part);
        } else {
            println!("Solution for day {} and year {} is not implemented yet", args.day, args.year);
        }
    } else {
        println!("Solution for year {} is not implemented yet", args.year);
    }
}

fn run_part<T: Day>(day: T, part: u32) {
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
