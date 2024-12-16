use clap::Parser;
use aoc::traits::Day;
use aoc::year2023;
use aoc::year2024;
use std::time::Instant;

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
    
    let mut day: Box<dyn Day> = match (args.year, args.day) {
        (2023, 1) => Box::new(year2023::day1::Day1::new()),
        (2023, 2) => Box::new(year2023::day2::Day2::new()),
        (2023, 3) => Box::new(year2023::day3::Day3::new()),
        (2023, 4) => Box::new(year2023::day4::Day4::new()),
        (2023, 5) => Box::new(year2023::day5::Day5::new()),
        (2023, 6) => Box::new(year2023::day6::Day6::new()),
        (2023, 7) => Box::new(year2023::day7::Day7::new()),
        (2023, 8) => Box::new(year2023::day8::Day8::new()),
        (2023, 9) => Box::new(year2023::day9::Day9::new()),
        (2023, 10) => Box::new(year2023::day10::Day10::new()),
        (2023, 11) => Box::new(year2023::day11::Day11::new()),
        (2023, 12) => Box::new(year2023::day12::Day12::new()),
        (2023, 13) => Box::new(year2023::day13::Day13::new()),
        (2023, 14) => Box::new(year2023::day14::Day14::new()),
        (2023, 15) => Box::new(year2023::day15::Day15::new()),
        (2023, 16) => Box::new(year2023::day16::Day16::new()),
        (2023, 17) => Box::new(year2023::day17::Day17::new()),
        (2023, 18) => Box::new(year2023::day18::Day18::new()),
        (2023, 19) => Box::new(year2023::day19::Day19::new()),
        (2023, 20) => Box::new(year2023::day20::Day20::new()),
        (2023, 21) => Box::new(year2023::day21::Day21::new()),
        (2023, 22) => Box::new(year2023::day22::Day22::new()),
        (2023, 23) => Box::new(year2023::day23::Day23::new()),
        (2023, 24) => Box::new(year2023::day24::Day24::new()),
        (2023, 25) => Box::new(year2023::day25::Day25::new()),

        (2024, 1) => Box::new(year2024::day1::Day1::new()),
        (2024, 2) => Box::new(year2024::day2::Day2::new()),
        (2024, 3) => Box::new(year2024::day3::Day3::new()),
        (2024, 4) => Box::new(year2024::day4::Day4::new()),
        (2024, 5) => Box::new(year2024::day5::Day5::new()),
        (2024, 6) => Box::new(year2024::day6::Day6::new()),
        (2024, 7) => Box::new(year2024::day7::Day7::new()),
        (2024, 8) => Box::new(year2024::day8::Day8::new()),
        (2024, 9) => Box::new(year2024::day9::Day9::new()),
        (2024, 10) => Box::new(year2024::day10::Day10::new()),
        (2024, 11) => Box::new(year2024::day11::Day11::new()),
        (2024, 12) => Box::new(year2024::day12::Day12::new()),
        (2024, 13) => Box::new(year2024::day13::Day13::new()),
        (2024, 14) => Box::new(year2024::day14::Day14::new()),
        (2024, 15) => Box::new(year2024::day15::Day15::new()),
        (2024, 16) => Box::new(year2024::day16::Day16::new()),

        _ => panic!("Solution for day {} and year {} is not implemented yet", args.day, args.year),
    };

    run_part(&mut day, args.part);
}

fn run_part(day: &mut Box<dyn Day>, part: u32) {
    let mut solution_fn: Box<dyn FnMut() -> u64> = match part {
        1 => Box::new(move || day.part_1()),
        2 => Box::new(move || day.part_2()),
        0 => {
            println!("Running both parts:");
            run_part(day, 1);
            run_part(day, 2);
            return;
        }
        _ => panic!("Invalid part: {}. Part must be 1, 2, or 0 for both.", part),
    };
    let start = Instant::now();
    let result = solution_fn();
    let duration = start.elapsed();
    
    println!(
        "Solution = {} ;; Took {:?}",
        result, duration
    );
}