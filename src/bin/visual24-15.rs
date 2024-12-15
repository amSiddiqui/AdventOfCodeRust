use aoc::year2024::day15::Day15;
use macroquad::prelude::*;

const FONT_SIZE: f32 = 14.;
const FONT_COLOR: Color = GRAY;
const ROBOT_COLOR: Color = RED;
const BLOCK_COLOR: Color = DARKBLUE;
const BACKGROUND_COLOR: Color = WHITE;
const TARGET_FPS: f32 = 30.0;
const X_PAD: f32 = 20.;
const Y_PAD: f32 = 20.;
const X_OFF: f32 = 6.;
const Y_OFF: f32 = 1.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Day15 Visualization".to_owned(),
        fullscreen: false,
        window_height: 700,
        window_width: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
pub async fn main() {
    let mut day = Day15::new();
    day.load();
    day.modify_grid();
    let movements = day.movement.clone();
    let total_movement = movements.len();
    let mut start = Day15::get_start(&day.grid).expect("Start not found");
    let mut i = 0;
    let mut accumulator = 0.0;
    let frame_time = 1.0 / TARGET_FPS;
    let grid_size = day.grid.len();
    let grid_w = day.grid[0].len();
    loop {
        clear_background(BACKGROUND_COLOR);

        for (y, line) in day.grid.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let color = match c {
                    '@' => ROBOT_COLOR,
                    '[' | ']' => BLOCK_COLOR,
                    _ => FONT_COLOR,
                };

                let info = format!("{}", c);
                draw_text(
                    &info,
                    (x as f32 * (FONT_SIZE - X_OFF)) + X_PAD,
                    (y as f32 * (FONT_SIZE - Y_OFF)) + Y_PAD,
                    FONT_SIZE,
                    color,
                );
            }
        }

        let info = format!("Step {i}/{total_movement}");
        draw_text(
            &info,
            X_PAD,
            (grid_size as f32 * (FONT_SIZE - Y_OFF)) + Y_PAD + 5.,
            FONT_SIZE,
            BLACK,
        );

        let delta = get_frame_time();
        accumulator += delta;


        let fps = 1. / frame_time;
        let info = format!("FPS {fps:.2}");
        draw_text(
            &info,
            X_PAD + (grid_w as f32 * (FONT_SIZE - X_OFF)) - 60.,
            (grid_size as f32 * (FONT_SIZE - Y_OFF)) + Y_PAD + 5.,
            FONT_SIZE,
            BLACK,
        );

        if i < movements.len() {
            if accumulator >= frame_time {
                accumulator -= frame_time;
                let dir = &movements[i];
                i += 1;
                start = day.step(*dir, start);
            }
        }

        next_frame().await;
    }
}
