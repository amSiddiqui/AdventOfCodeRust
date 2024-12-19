# Advent of Code Solutions in Rust

This repository contains my solutions for the [Advent of Code](https://adventofcode.com/) challenges, written in Rust. Advent of Code is an annual set of Christmas-themed programming challenges that cover a variety of topics and difficulty levels.

## Parallelization with Rayon

To enhance performance, especially for computationally intensive tasks, these solutions utilize the [Rayon](https://crates.io/crates/rayon). Rayon is a data-parallelism library for Rust, which makes it straightforward to convert sequential computations into parallel ones. It is particularly effective for CPU-bound tasks and can significantly improve the execution time of solutions.

## Structure

The solutions are organized by year and day. Each year has its own directory, and within each year's directory, there are Rust files for each day of the challenge.

### Input Data Structure

Each challenge expects its input data to be placed in a specific folder structure under the `data` directory. The expected structure is as follows:

```
data/<YEAR>/day<DAY>
```

For example, for Day 19 of the 2024 challenges, the input file should be located at:
```
data/year2024/day19
```

The program reads this input file as demonstrated in the following code snippet:
```rust
impl Day19 {
    pub fn new() -> Self {
        let data = fs::read_to_string("data/year2024/day19").expect("Cannot read data file");
        // -- snip --
    }
}
```

Make sure to either update the input file path in the code or place the input file in the correct folder before running the solution.

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

## Visualizations

Some challenges include visualizations to better understand or demonstrate the problem-solving process. Visualizations are implemented as separate binaries under the `src/bin` folder and can be run independently.

### Running a Visualization

To run a visualization, use the following command:

```bash
cargo run --bin <BINARY_NAME>
```

### Example
For the visualization of Day 14 in 2024:

```bash
cargo run --bin visual24-14
```

The visualization binaries are organized in `src/bin` and are implemented using the `macroquad` library. The visualization for each day will open a graphical window with interactive or dynamic elements showcasing the problem and its solution.

## Contributing
While this is primarily a personal project for learning and fun, suggestions and improvements are welcome. Please feel free to open an issue or pull request.