extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use rand::Rng;

extern crate timer;
extern crate chrono;
use std::sync::{Arc, Mutex};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const SQUARE_SIZE: usize = 5;

fn main() {
    let timer = timer::Timer::new();
    // Number of times the callback has been called.
    let count = Arc::new(Mutex::new(0));

    let guard = {
        let count = count.clone();
        timer.schedule_repeating(chrono::Duration::milliseconds(100), move || {
          *count.lock().unwrap() += 1;
        })
    };


    let grid_width = WIDTH/SQUARE_SIZE;
    let grid_height = HEIGHT/SQUARE_SIZE;
    // let mut grid = random_grid(grid_width, grid_height);
    let mut grid = init_grid(grid_width, grid_height);
    grid = put(glider_gun(), grid, 10, 10);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut prev_count: i32 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, j) in buffer.iter_mut().enumerate() {
            let y = i / WIDTH;
            let x = i % WIDTH;

            let grid_y = y / SQUARE_SIZE;
            let grid_x = x / SQUARE_SIZE;

            if grid[grid_y][grid_x] > 0 {
                *j = 0xCCAADD00;
            } else {
                *j = 0x11111100;
            }

            let count_result = *count.lock().unwrap();
            if count_result > prev_count {
                prev_count = count_result;

                grid = update_grid(grid, grid_width, grid_height);
            }

            
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn init_grid(grid_width: usize, grid_height: usize) -> Vec<Vec<u8>> {
    let mut new_grid: Vec<Vec<u8>> = Vec::with_capacity(grid_height);
    for y in 0..grid_height {
        let v: Vec<u8> =  vec![0; grid_width];
        new_grid.push(v);
    }
    new_grid
}

fn random_grid(grid_width: usize, grid_height: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(grid_height);
    for _ in 0..grid_height {
        let mut v: Vec<u8> = (0..grid_width).map(|_| {
            //0
            rng.gen_range(0..2)
        }).collect();
        grid.push(v);
    }
    grid
}

fn update_grid(grid: Vec<Vec<u8>>, grid_width: usize, grid_height: usize) -> Vec<Vec<u8>> {
    let mut new_grid = init_grid(grid_width, grid_height);

    for y in 0..grid_height {
        for x in 0..grid_width {
            

            let mut x_start = 0;
            if x > 0 {
                x_start = x-1;
            }

            let mut x_end = x+2;
            if x_end > grid_width {
                x_end = grid_width;
            }

            let mut y_start = 0;
            if y > 0 {
                y_start = y-1;
            }

            let mut y_end = y+2;
            if y_end > grid_height {
                y_end = grid_height;
            }

            let mut neighbours: u8 = 0;
            for xx in x_start..x_end {
                for yy in y_start..y_end {
                    if ! ( xx==x && yy==y ) {
                        if grid[yy][xx] == 1 {
                            neighbours += 1;
                        }
                    }
                }
            }

            new_grid[y][x] = grid[y][x];
            if grid[y][x] == 1 {
                if neighbours < 2 || neighbours > 3 {
                    new_grid[y][x] = 0;
                }
            } else {     
                if (neighbours == 3 ) {
                  new_grid[y][x] = 1;
                }
            }
        }
    }

    new_grid
}


fn put(figure: Vec<Vec<u8>>, grid: Vec<Vec<u8>>, offset_x: usize, offset_y: usize) -> Vec<Vec<u8>> {
    let mut new_grid: Vec<Vec<u8>> = Vec::with_capacity(grid.len());
    for y in 0..grid.len() {
        let v: Vec<u8> =  vec![0; grid[0].len()];
        new_grid.push(v);
    }

    for x in 0..figure.len() {
        for y in 0..figure[0].len() {
            new_grid[x+offset_x][y+offset_y] = figure[x][y];
        }
    }
    new_grid
}

fn glider_gun() -> Vec<Vec<u8>> {
    let mut g: Vec<Vec<u8>> = Vec::with_capacity(38);
    for _ in 0..10 {
        let v: Vec<u8> =  vec![0; 38];
        g.push(v);
    }

    g[3][36] = 1;
    g[4][36] = 1;
    g[3][35] = 1;
    g[4][35] = 1;
    
    g[1][25] = 1;
    g[2][25] = 1;
    g[2][23] = 1;
    g[3][21] = 1;
    g[4][21] = 1;
    g[5][21] = 1;
    g[3][22] = 1;
    g[4][22] = 1;
    g[5][22] = 1;
    g[6][23] = 1;
    g[6][25] = 1;
    g[7][25] = 1;
    
    g[6][18] = 1;
    g[5][17] = 1;
    g[6][17] = 1;
    g[7][17] = 1;
    g[4][16] = 1;
    g[8][16] = 1;
    g[6][15] = 1;
    g[3][14] = 1;
    g[3][13] = 1;
    g[9][14] = 1;
    g[9][13] = 1;
    g[4][12] = 1;
    g[8][12] = 1;
    g[5][11] = 1;
    g[6][11] = 1;
    g[7][11] = 1;
    
    g[5][1] = 1;
    g[5][2] = 1;
    g[6][1] = 1;
    g[6][2] = 1;

    g
}