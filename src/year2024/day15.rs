use std::fs;

use crate::traits::Day;

pub struct Day15 {
    grid: Vec<Vec<char>>,
    movement: Vec<char>,
}

impl Day15 {
    pub fn new() -> Self {
        Day15 { grid: Vec::new(), movement: Vec::new() }
    }

    pub fn load(&mut self) {
        let data_str = fs::read_to_string("data/year2024/day15").expect("Cannot read data");
        let mut parts = data_str.split("\n\n");
        let grid: Vec<Vec<char>> = parts
            .next()
            .unwrap()
            .lines()
            .map(|l| l.chars().collect())
            .collect();
        let movement = parts.next().unwrap().chars().collect();
        self.grid = grid;
        self.movement = movement;
    } 

    pub fn modify_grid(&mut self) {
        let mut new_grid = Vec::new();

        for (y, line) in self.grid.iter().enumerate() {
            new_grid.push(Vec::new());
            for (_, ch) in line.iter().enumerate() {
                match ch {
                    '.' => {
                        new_grid[y].push('.');
                        new_grid[y].push('.')
                    }
                    '#' => {
                        new_grid[y].push('#');
                        new_grid[y].push('#')
                    }
                    'O' => {
                        new_grid[y].push('[');
                        new_grid[y].push(']')
                    }
                    _ => {
                        new_grid[y].push('@');
                        new_grid[y].push('.')
                    }
                }
            }
        }

        self.grid = new_grid;
    }

    pub fn get(&self, x: i32, y: i32) -> char {
        self.grid[y as usize][x as usize]
    }
}

fn get_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (y, l) in grid.iter().enumerate() {
        for (x, ch) in l.iter().enumerate() {
            if *ch == '@' {
                return Some((x, y));
            }
        }
    }
    None
}

impl Day for Day15 {
    fn part_1(&mut self) -> u64 {
        self.load();
        let mut start = get_start(&self.grid).expect("Start not found");
        for dir in &self.movement {
            let (dx, dy): (i32, i32) = match dir {
                '^' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                _ => (0, 1),
            };
            let (nxi, nyi) = (start.0 as i32 + dx, start.1 as i32 + dy);
            let (nx, ny) = (nxi as usize, nyi as usize);
            if self.grid[ny][nx] == '#' {
                continue;
            }

            if self.grid[ny][nx] == '.' {
                self.grid[ny][nx] = '@';
                self.grid[start.1][start.0] = '.';
                start = (nx, ny);
                continue;
            }

            let (mut bxi, mut byi) = (nxi, nyi);
            while self.grid[byi as usize][bxi as usize] == 'O' {
                bxi += dx;
                byi += dy;
            }

            if self.grid[byi as usize][bxi as usize] == '#' {
                continue;
            }

            while self.grid[byi as usize][bxi as usize] != '@' {
                self.grid[byi as usize][bxi as usize] = 'O';
                bxi -= dx;
                byi -= dy;
            }

            self.grid[byi as usize][bxi as usize] = '.';
            self.grid[(byi + dy) as usize][(bxi + dx) as usize] = '@';
            start = ((bxi + dx) as usize, (byi + dy) as usize);
        }

        let mut total = 0;
        for (y, l) in self.grid.iter().enumerate() {
            for (x, ch) in l.iter().enumerate() {
                if *ch == 'O' {
                    total += 100 * y + x;
                }
            }
        }
        total as u64
    }

    fn part_2(&mut self) -> u64 {
        self.load();
        self.modify_grid();
        let mut start = get_start(&self.grid).expect("Start not found");
        for dir in &self.movement {
            let (dx, dy): (i32, i32) = match dir {
                '^' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                _ => (0, 1),
            };

            let is_vertical = *dir == 'v' || *dir == '^';

            let (nxi, nyi) = (start.0 as i32 + dx, start.1 as i32 + dy);
            let (nx, ny) = (nxi as usize, nyi as usize);
            if self.grid[ny][nx] == '#' {
                continue;
            }

            if self.grid[ny][nx] == '.' {
                self.grid[ny][nx] = '@';
                self.grid[start.1][start.0] = '.';
                start = (nx, ny);
                continue;
            }

            if !is_vertical {
                let (mut bxi, mut byi) = (nxi, nyi);
                while self.grid[byi as usize][bxi as usize] == '['
                    || self.grid[byi as usize][bxi as usize] == ']'
                {
                    bxi += dx;
                    byi += dy;
                }

                if self.grid[byi as usize][bxi as usize] == '#' {
                    continue;
                }

                while self.grid[byi as usize][bxi as usize] != '@' {
                    let (pbxi, pbyi) = (bxi - dx, byi - dy);
                    self.grid[byi as usize][bxi as usize] = self.grid[pbyi as usize][pbxi as usize];

                    bxi -= dx;
                    byi -= dy;
                }

                self.grid[byi as usize][bxi as usize] = '.';
                self.grid[(byi + dy) as usize][(bxi + dx) as usize] = '@';
                start = ((bxi + dx) as usize, (byi + dy) as usize);
                continue;
            }

            let mut boundary = vec![];
            let mut byi = nyi;
            boundary.push(nxi);
            if self.get(nxi, nyi) == ']' {
                boundary.insert(0, nxi - 1);
            } else {
                boundary.push(nxi + 1);
            }
            let mut blocked = false;
            let mut all_boundaries = vec![boundary.clone()];
            loop {
                byi += dy;
                let mut all_free_space = true;
                for bxi in &boundary {
                    if self.get(*bxi, byi) == '#' {
                        blocked = true;
                        break;
                    }
                    if self.get(*bxi, byi) != '.' {
                        all_free_space = false;
                    }
                }
                if blocked || all_free_space {
                    break;
                }

                while self.get(boundary[0], byi) == '.' {
                    boundary.remove(0);
                }

                while self.get(boundary[boundary.len() - 1], byi) == '.' {
                    boundary.remove(boundary.len() - 1);
                }

                if self.get(boundary[0], byi) == ']' {
                    boundary.insert(0, boundary[0] - 1);
                }
                if self.get(boundary[boundary.len() - 1], byi) == '[' {
                    boundary.push(boundary[boundary.len() - 1] + 1);
                }

                all_boundaries.push(boundary.clone());
            }
            if blocked {
                continue;
            }
            for b in all_boundaries.iter().rev() {
                let pyi = byi - dy;
                for xi in b {
                    self.grid[byi as usize][*xi as usize] = self.get(*xi, pyi);
                }
                for xi in b {
                    self.grid[pyi as usize][*xi as usize] = '.';
                }
                byi = pyi;
            }
            self.grid[byi as usize][start.0] = '@';
            self.grid[(byi - dy) as usize][start.0] = '.';
            start = (start.0, byi as usize);
        }
        println!("===================Final====================");
        for l in self.grid.iter() {
            for c in l.iter() {
                print!("{c}");
            }
            println!();
        }
        println!("======================================");

        let mut total = 0;
        for (y, l) in self.grid.iter().enumerate() {
            for (x, ch) in l.iter().enumerate() {
                if *ch == '[' {
                    total += 100 * y + x;
                }
            }
        }

        total as u64
    }
}
