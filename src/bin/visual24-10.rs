use std::collections::HashSet;

use aoc::year2024::day10::Day10;
use macroquad::prelude::*;

const FONT_SIZE: f32 = 14.;
const FONT_COLOR: Color = GRAY;
const BACKGROUND_COLOR: Color = WHITE;
const TARGET_FPS: f32 = 15.0;

fn graph_traverse(
    graph: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    expected: u8,
    visited: &mut HashSet<(usize, usize)>,
    vis_arr: &mut Vec<(usize, usize)>,
) -> u64 {
    if visited.contains(&(x, y)) {
        return 0;
    }
    visited.insert((x, y));
    vis_arr.push((x, y));
    if graph[y][x] == 9 {
        return 1;
    }
    let mut score = 0;
    // west
    if x > 0 && graph[y][x - 1] == expected {
        score += graph_traverse(graph, x - 1, y, expected + 1, visited, vis_arr);
    }
    // east
    if x < graph[0].len() - 1 && graph[y][x + 1] == expected {
        score += graph_traverse(graph, x + 1, y, expected + 1, visited, vis_arr);
    }
    // north
    if y > 0 && graph[y - 1][x] == expected {
        score += graph_traverse(graph, x, y - 1, expected + 1, visited, vis_arr);
    }
    // south
    if y < graph.len() - 1 && graph[y + 1][x] == expected {
        score += graph_traverse(graph, x, y + 1, expected + 1, visited, vis_arr);
    }
    score
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Day10 Visualization".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
pub async fn main() {
    let day = Day10::new();
    let colors = [
        BLUE, GREEN, RED, SKYBLUE, PURPLE, ORANGE, DARKPURPLE, DARKBLUE,
    ];
    let mut data: Vec<Vec<(usize, usize, Color)>> = day
        .starting
        .iter()
        .map(|start| {
            let mut visited = HashSet::new();
            let mut vis_arr = vec![];
            graph_traverse(&day.lines, start.0, start.1, 1, &mut visited, &mut vis_arr);
            vis_arr
                .into_iter()
                .map(|v| (v.0, v.1, Color::new(0.0, 0., 0., 0.)))
                .collect()
        })
        .collect();
    let mut i = 0;
    let mut ci = 0;
    let mut ti = 0;
    let frame_time = 1.0 / TARGET_FPS;
    let mut accumulator = 0.0;

    data[i][ti].2 = colors[ci];
    loop {
        let delta = get_frame_time();
        accumulator += delta;
        clear_background(BACKGROUND_COLOR);
        for (y, line) in day.lines.iter().enumerate() {
            for (x, digit) in line.iter().enumerate() {
                let info = format!("{}", digit);
                draw_text(
                    &info,
                    x as f32 * FONT_SIZE,
                    y as f32 * FONT_SIZE,
                    FONT_SIZE,
                    FONT_COLOR,
                );
            }
        }


        for j in 0..=i {
            for point in &data[j] {
                let info = format!("{}", day.lines[point.1][point.0]);
                draw_text(
                    &info,
                    point.0 as f32 * FONT_SIZE,
                    point.1 as f32 * FONT_SIZE,
                    FONT_SIZE,
                    point.2,
                );
            }
        }

        if accumulator >= frame_time {
            accumulator -= frame_time;
            data[i][ti].2 = colors[ci];
            ti += 1;
            if ti == data[i].len() {
                ti = 0;
                i = (i + 1) % data.len();
                ci = (ci + 1) % colors.len();
            }
        }

        // Proceed to the next frame
        next_frame().await;
    }
}
