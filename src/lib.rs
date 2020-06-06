mod utils;

extern crate js_sys;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in row {
                let symbol = if cell == Cell::Alive { 'O' } else { '-' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn fill_random(&mut self, p_alive: f64) {
        for i in 0..self.cells.len() {
            self.cells[i] = if js_sys::Math::random() < p_alive {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(x, y);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, n) if (n <= 1) => Cell::Dead,
                    (Cell::Alive, n) if (n >= 2 && n <= 3) => Cell::Alive,
                    (Cell::Alive, n) if (n >= 4) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => cell,
                };
                next_cells[idx] = next_cell;
            }
        }
        self.cells = next_cells;
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut adj_count = 0;
        for delta_x in [self.width - 1, 0, 1].iter().cloned() {
            for delta_y in [self.height - 1, 0, 1].iter().cloned() {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }
                let neighbour_x = (x + delta_x) % self.width;
                let neighbour_y = (y + delta_y) % self.height;
                adj_count += self.cells[self.get_index(neighbour_x, neighbour_y)] as u8;
            }
        }

        adj_count
    }
}
