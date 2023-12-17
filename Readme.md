# Advent of Code Solutions in Rust

This repository contains my solutions for the [Advent of Code](https://adventofcode.com/) challenges, written in Rust. Advent of Code is an annual set of Christmas-themed programming challenges that cover a variety of topics and difficulty levels.


## Parallelization with Rayon

To enhance performance, especially for computationally intensive tasks, these solutions utilize the [Rayon](https://crates.io/crates/rayon). Rayon is a data-parallelism library for Rust, which makes it straightforward to convert sequential computations into parallel ones. It is particularly effective for CPU-bound tasks and can significantly improve the execution time of solutions.


## Structure

The solutions are organized by year and day. Each year has its own directory, and within each year's directory, there are rust files for each day of the challenge.

## Running the Solutions

To run a solution, use the `cargo` command-line tool. The binary allows you to specify the year, day, and part of the challenge you want to execute.

### Usage

```bash
cargo run --release -- --year <YEAR> --day <DAY> --part <PART>
```
### Example
```
cargo run --release -- --year 2023 --day 1 --part 1
cargo run --release -- --help
```

### Contributing
While this is primarily a personal project for learning and fun, suggestions and improvements are welcome. Please feel free to open an issue or pull request.
