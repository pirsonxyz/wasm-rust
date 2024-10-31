use rand::{seq::SliceRandom, thread_rng, Rng};
use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Dead = 0,
    Alive = 1,
}
struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
impl Universe {
    fn new(width: u32, height: u32) -> Self {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Self {
            width,
            height,
            cells,
        }
    }
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbor = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbor) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    fn render(&self, drawer: &mut impl RaylibDraw) {
        let colors = vec![Color::WHITE, Color::RED, Color::GREEN, Color::BLUE];
        let mut rng = thread_rng();
        let mut offset = rng.gen_range(-25..25);
        for line in self.cells.as_slice().chunks(self.width as usize) {
            let random = rng.gen_range(0..25);
            for &cell in line {
                if cell == Cell::Dead {
                    continue;
                } else {
                    drawer.draw_rectangle(
                        rng.gen_range(0..1280) + offset,
                        rng.gen_range(0..720) + offset,
                        8,
                        8,
                        colors.choose(&mut rng).unwrap(),
                    )
                };
                offset += random
            }
        }
    }
}

fn main() {
    let width = 1280;
    let height = 720;
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Rust + Raylib + Wasm")
        .build();
    let mut universe = Universe::new(width as u32, height as u32);
    rl.set_target_fps(1);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        universe.render(&mut d);
        universe.tick();
    }
}
