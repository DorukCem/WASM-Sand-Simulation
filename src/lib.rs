use wasm_bindgen::prelude::*;
mod utils;

/// Compile: wasm-pack build --target bundler
/// Run: cd site
///      npm run serve

/// Javascript can only store C style enums memory buffer
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Dead = 0,
    Sand = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    id : CellType,
    has_been_updated: bool,
}

impl Cell {
    fn toggle(&mut self) {
        self.id = match self.id {
            CellType::Dead => CellType::Sand,
            CellType::Sand => CellType::Dead,
        };
    }

    fn new(ct: CellType) -> Self {
        return Cell { id: ct, has_been_updated: true }
    }
}


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    ///! Dont need
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
                if let CellType::Dead = self.cells[idx].id {}
                else {count += 1;}
                
            }
        }
        count
    }

    /// Get the dead and Sand values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be Sand in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx].id = CellType::Sand;
        }
    }
    
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    ///! Change functionality
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell.id, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (CellType::Sand, x) if x < 2 => CellType::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (CellType::Sand, 2) | (CellType::Sand, 3) => CellType::Sand,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (CellType::Sand, x) if x > 3 => CellType::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (CellType::Dead, 3) => CellType::Sand,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx].id = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook(); // If our code panics, we want informative error messages to appear in the developer console

        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::new(CellType::Sand)
                    
                } else {
                    Cell::new(CellType::Dead)
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render_to_console(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// This method will be called by javascript to get the memory buffer of our cells
    pub fn cells(&self) -> *const CellType {
        self.cells.iter().map(|&c| c.id).collect::<Vec<CellType>>().as_ptr()
    }

    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::new(CellType::Dead)).collect();
    }

    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::new(CellType::Dead)).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell.id == CellType::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
