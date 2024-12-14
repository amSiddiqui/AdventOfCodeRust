
use aoc::year2024::day14::Day14;
use macroquad::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Cell { x, y }
    }

    pub fn to_screen_position(&self, cell_size: f32) -> (f32, f32) {
        (self.x as f32 * cell_size, self.y as f32 * cell_size)
    }
}

pub struct Grid {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub base_color: Color,
    pub guard_color: Color,
    pub grid_line_color: Color,
}

impl Grid {
    pub fn new(
        grid_width: usize,
        grid_height: usize,
        cell_size: f32,
        base_color: Color,
        guard_color: Color,
        grid_line_color: Color,
    ) -> Self {
        Grid {
            grid_width,
            grid_height,
            cell_size,
            base_color,
            guard_color,
            grid_line_color,
        }
    }

    pub fn draw_grid(&self, guard_position: &[(i32, i32)]) {
        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let cell = Cell::new(x, y);
                let (screen_x, screen_y) = cell.to_screen_position(self.cell_size);
                draw_rectangle(
                    screen_x,
                    screen_y,
                    self.cell_size,
                    self.cell_size,
                    self.base_color,
                );
            }
        }

        for (x, y) in guard_position {
            let (screen_x, screen_y) = (*x as f32 * self.cell_size, *y as f32 * self.cell_size);

            draw_rectangle(
                screen_x,
                screen_y,
                self.cell_size,
                self.cell_size,
                self.guard_color,
            );
        }

        for x in 0..=self.grid_width {
            let pos_x = x as f32 * self.cell_size;
            draw_line(
                pos_x,
                0.0,
                pos_x,
                self.grid_height as f32 * self.cell_size,
                1.0,
                self.grid_line_color,
            );
        }

        for y in 0..=self.grid_height {
            let pos_y = y as f32 * self.cell_size;
            draw_line(
                0.0,
                pos_y,
                self.grid_height as f32 * self.cell_size,
                pos_y,
                1.0,
                self.grid_line_color,
            );
        }
    }
}



const GRID_WIDTH: usize = 101;
const GRID_HEIGHT: usize = 103;
const CELL_SIZE: f32 = 5.0;

const COLOR_BACKGROUND: Color = WHITE;
const COLOR_GRID_LINES: Color = Color::new(0.9, 0.9, 0.9, 0.5);
const COLOR_GUARD: Color = RED;
const COLOR_TEXT: Color = BLACK;

fn window_conf() -> Conf {
    Conf {
        window_title: "Day14 Visualization".to_owned(),
        fullscreen: false,
        window_height: GRID_HEIGHT as i32 * CELL_SIZE as i32 + 20,
        window_width: GRID_WIDTH as i32 * CELL_SIZE as i32 + 10,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
pub async fn main() {
    let grid = Grid::new(
        GRID_WIDTH,
        GRID_HEIGHT,
        CELL_SIZE,
        COLOR_BACKGROUND,
        COLOR_GUARD,
        COLOR_GRID_LINES
    );

    let mut day = Day14::new();
    day.load();
    let mut second = 1;
    loop {
        clear_background(COLOR_BACKGROUND);
        let guard_position: Vec<(i32, i32)> = day.data.iter().map(|&(pos, _)| pos).collect();
        grid.draw_grid(&guard_position);
        let info = format!("Second {second}");
        draw_text(&info, 10.0, (GRID_HEIGHT as f32 * grid.cell_size) + 12., 20.0, COLOR_TEXT);
        if !day.is_tree() {
            day.step();
            second += 1;
        }

        next_frame().await;
    }
}

