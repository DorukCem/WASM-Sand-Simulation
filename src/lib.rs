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
    id: CellType,
    has_been_updated: bool,
}

impl Cell {
    fn set_cell(&mut self, ct: CellType) {
        self.id = ct;
    }

    fn update_cell(&mut self, ref_cell : Cell) {
        self.id = ref_cell.id;
        self.has_been_updated = true;
    }

    fn kill_cell(&mut self) {
        self.id = CellType::Dead;
    }

    fn new(ct: CellType) -> Self {
        return Cell {
            id: ct,
            has_been_updated: true,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    tick_count: u8,
}

const TICK_INTERVAL: u8 = 1;

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn is_empty_and_inbound(&self, idx: usize) -> bool {
        idx < self.cells.len() && self.cells[idx].id == CellType::Dead
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
    
    fn update_sand(&mut self ,row: u32, col: u32,) {
        let idx = self.get_index(row, col);
        let down = self.get_index(row + 1, col);
        let left = self.get_index(row + 1, col - 1);
        let right = self.get_index(row + 1, col + 1);
    
        let new_idx = if self.is_empty_and_inbound(down) {
            down
        } else if self.is_empty_and_inbound(left) {
            left
        } else if self.is_empty_and_inbound(right) {
            right
        } else {
            return;
        };
        
        let copy_current_cell = self.cells[idx];
        self.cells[new_idx].update_cell(copy_current_cell);
        
        self.cells[idx].kill_cell();

    }
}


/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        // ! We might not need this part if we decide on 60 tick per secon
        self.tick_count = (self.tick_count + 1) % TICK_INTERVAL;
        if self.tick_count != 0 {
            return;
        }

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                if cell.has_been_updated {
                    utils::log!("cont");
                    continue;
                }
                match cell.id {
                    CellType::Sand => self.update_sand(row, col),
                    CellType::Dead => (),
                }
            }
        }

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                self.cells[idx].has_been_updated = false;
            }
        }
    }

    pub fn new() -> Universe {
        utils::set_panic_hook(); // If our code panics, we want informative error messages to appear in the developer console

        let width = 64;
        let height = 64;
        let tick_count = 0;

        let cells = (0..width * height)
            .map(|_i| Cell::new(CellType::Dead))
            .collect();

        Universe {
            width,
            height,
            cells,
            tick_count,
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
        self.cells
            .iter()
            .map(|&c| c.id)
            .collect::<Vec<CellType>>()
            .as_ptr()
    }

    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height)
            .map(|_i| Cell::new(CellType::Dead))
            .collect();
    }

    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height)
            .map(|_i| Cell::new(CellType::Dead))
            .collect();
    }

    pub fn set_cell(&mut self, row: u32, column: u32, ct: CellType) {
        let idx = self.get_index(row, column);
        self.cells[idx].set_cell(ct);
    }
}

use std::fmt;
// ? Can add more colors as I add more elements
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell.id == CellType::Dead {
                    '◻'
                } else {
                    '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
