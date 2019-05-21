extern crate minifb;
use std::mem;
use std::time::SystemTime;
use std::usize;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

const CELL_SIZE: usize = 8;
const GRID_SIZE: usize = 100;

const WIDTH: usize = CELL_SIZE * GRID_SIZE;
const HEIGHT: usize = CELL_SIZE * GRID_SIZE;

const CELL_WIDTH: usize = GRID_SIZE;
const CELL_HEIGHT: usize = GRID_SIZE;

struct Grid {
    front_buffer: Vec<bool>,
    back_buffer: Vec<bool>,
}

impl Grid {
    fn compute_neighbour(&self, x: usize, y: usize) -> u32 {
        let mut count = 0u32;

        if self.get_cell_value(x.wrapping_sub(1), y.wrapping_sub(1)) {
            count += 1;
        }
        if self.get_cell_value(x, y.wrapping_sub(1)) {
            count += 1;
        }
        if self.get_cell_value(x.wrapping_add(1), y.wrapping_sub(1)) {
            count += 1;
        }
        if self.get_cell_value(x.wrapping_sub(1), y) {
            count += 1;
        }
        if self.get_cell_value(x.wrapping_add(1), y) {
            count += 1;
        }
        if self.get_cell_value(x.wrapping_sub(1), y.wrapping_add(1)) {
            count += 1;
        }
        if self.get_cell_value(x, y.wrapping_add(1)) {
            count += 1;
        }
        if self.get_cell_value(x.wrapping_add(1), y.wrapping_add(1)) {
            count += 1;
        }

        count
    }

    fn get_xy(value: usize, x: &mut usize, y: &mut usize) {
        *x = value % GRID_SIZE;
        *y = value / GRID_SIZE;
    }

    fn get_cell_value(&self, x: usize, y: usize) -> bool {
        if !Grid::is_in_bounds(x, y) {
            return false;
        }

        self.front_buffer[y * GRID_SIZE + x]
    }

    fn is_in_bounds(x: usize, y: usize) -> bool {
        x < GRID_SIZE && y < GRID_SIZE
    }

    fn swap_buffers(&mut self) {
        mem::swap(&mut self.front_buffer, &mut self.back_buffer);
    }

    fn update(&mut self) {
        for (index, _) in self.front_buffer.iter().enumerate() {
            let mut x = 0usize;
            let mut y = 0usize;
            Grid::get_xy(index, &mut x, &mut y);

            self.back_buffer[index] = match self.compute_neighbour(x, y) {
                2 => self.front_buffer[index],
                3 => true,
                _ => false,
            };
        }

        self.swap_buffers();
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut grid = Grid {
        front_buffer: vec![false; CELL_WIDTH * CELL_HEIGHT],
        back_buffer: vec![false; CELL_WIDTH * CELL_HEIGHT],
    };

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut clock = SystemTime::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_mouse_pos(MouseMode::Discard).map(|mouse| {
            // map mouse position to grid cell position
            let cell_x = mouse.0 as usize / CELL_SIZE;
            let cell_y = mouse.1 as usize / CELL_SIZE;

            if window.get_mouse_down(MouseButton::Left) {
                grid.front_buffer[cell_y * CELL_WIDTH + cell_x] = true;
            } else if window.get_mouse_down(MouseButton::Right) {
                grid.front_buffer[cell_y * CELL_WIDTH + cell_x] = false;
            }
        });

        if window.is_key_down(Key::Space) {
            match clock.elapsed() {
                Ok(d) => {
                    if d.as_millis() >= 100u128 {
                        grid.update();
                        clock = SystemTime::now();
                    }
                }
                Err(err) => {
                    println!("An error occurred: {}", err);
                }
            }
        }

        for (index, cell) in buffer.iter_mut().enumerate() {
            let x = index % WIDTH;
            let y = index / WIDTH;

            let cell_x = x / CELL_SIZE;
            let cell_y = y / CELL_SIZE;

            *cell = if grid.front_buffer[cell_y * CELL_WIDTH + cell_x] {
                0xFFFFFF
            } else {
                0
            };
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
