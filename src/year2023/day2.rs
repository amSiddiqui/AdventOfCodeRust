use std::collections::HashMap;
use std::fs;
use crate::traits::Day;
use lazy_static::lazy_static;
use rayon::prelude::*;

lazy_static! {
    static ref LIMITS: HashMap<String, i32> = {
        let mut limit: HashMap<String, i32> = HashMap::new();
        limit.insert(String::from("red"), 12);
        limit.insert(String::from("green"), 13);
        limit.insert(String::from("blue"), 14);
        limit
    };
}

pub struct Day2 {
    lines: Vec<String>,
}

impl Day2 {
    pub fn new() -> Self {
        let lines = fs::read_to_string("data/day2")
            .unwrap_or_else(|er| panic!("Error reading the file. Err {er}"))
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>();
        Day2 {
            lines
        }
    }
}

impl Day for Day2 {
    fn part_1(&self) -> u64 {
        self.lines.par_iter().enumerate().filter_map(|(id, line)| {
            let mut parts = line.splitn(2, ':').skip(1);
            let is_game_possible = parts.next()
                .expect("Games data not present")
                .split(';')
                .map(|x| x.trim())
                .all(|game|
                    game.split(',')
                        .all(|game| {
                            let mut game_part = game.trim().splitn(2, ' ');
                            let count = game_part.next()
                                .expect("Game count part not present")
                                .parse::<i32>()
                                .expect("Number malformed");
                            let ball = game_part.next()
                                .expect("Game ball part not present")
                                .to_string();
                            let limit = *LIMITS.get(&ball).unwrap_or_else(|| panic!("Unrecognized color {ball}"));
                            count <= limit
                        })
                );
            if is_game_possible {
                Some(id as u64 + 1)
            } else {
                None
            }
        }).sum::<u64>()
    }

    fn part_2(&self) -> u64 {
        self.lines.par_iter().map(|line| {
            let game_line = line.split_once(':').map(|x| x.1).expect("Game line not present");
            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;
            for games in game_line.trim().split(';') {
                for balls in games.trim().split(',') {
                    let (count, ball) = balls.trim().split_once(' ').expect("Count and ball not present");
                    let count = count.parse::<i32>().expect("Count malformed");
                    if ball == "red" && count > min_red {
                        min_red  = count;
                    }
                    if ball == "green" && count > min_green {
                        min_green  = count;
                    }
                    if ball == "blue" && count > min_blue {
                        min_blue  = count;
                    }
                }
            }
            min_red * min_blue * min_green
        }).sum::<i32>() as u64
    }

}
