use wasm_bindgen::prelude::*;
use web_sys::js_sys::Math::random;
mod utils;


const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;
const SPREAD_FACTOR: u32 = 3;

/// Javascript can only store C style enums memory buffer
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Dead = 0,
    Sand = 1,
    Water = 2,
    Rock = 3,
}

#[derive(PartialEq, Eq)]
enum Phase {
    Dead,
    Solid,
    Liquid,
    Immovable
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    id: CellType,
    energy: u32,
    has_been_updated: bool,
}

impl Cell {
    fn set_cell(&mut self, ct: CellType) {
        self.id = ct;
    }

    fn new(ct: CellType) -> Self {
        return Cell {
            id: ct,
            energy: 0,
            has_been_updated: false,
        };
    }

    fn phase(&self) -> Phase {
        let id_as_num = self.id as u8;
        if id_as_num == 0 {
            return Phase::Dead;
        }
        if id_as_num < 2 {
            return Phase::Solid;
        }

        if id_as_num < 3 {
            return Phase::Liquid;
        }

        return Phase::Immovable

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

    fn is_empty_and_inbound(&self, row: u32, col: u32) -> Option<(u32, u32)> {
        if !(row < self.height && col < self.width) {
            return None; // This also works for -1 which gets converted to u32MAX
        }
        let idx = self.get_index(row, col);
        if self.cells[idx].id == CellType::Dead {
            return Some((row, col));
        }
        None
    }

    fn is_phase(&self, row: u32, col: u32, ele: Phase) -> Option<(u32, u32)> {
        if !(row < self.height && col < self.width) {
            return None; // This also works for -1 which gets converted to u32MAX
        }
        let idx = self.get_index(row, col);
        if self.cells[idx].phase() == ele {
            return Some((row, col));
        }
        None
    }

    /// Get the dead and Sand values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    fn find_valid_positions(&self, positions: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        positions
            .iter()
            .map(|x| self.is_empty_and_inbound(x.0, x.1))
            .take_while(|x| x.is_some())
            .flatten()
            .collect::<Vec<_>>()
    }

    fn find_valid_positions_for_solid(&self, positions: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        positions
            .iter()
            .map(|x| {
                self.is_empty_and_inbound(x.0, x.1)
                    .or(self.is_phase(x.0, x.1, Phase::Liquid))
            })
            .take_while(|x| x.is_some())
            .flatten()
            .collect::<Vec<_>>()
    }


    fn switch_cells(&mut self, old_idx: usize, new_idx: usize) {
        self.cells.swap(old_idx, new_idx)
    }


    fn update_sand(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].has_been_updated = true;
        let cell_energy = self.cells[idx].energy / 4;

        let downwards_positions: Vec<_> = (1..=cell_energy + 1).map(|i| (row + i, col)).collect();
        let left_positions = vec![(row + 1, col - 1)];
        let right_positions = vec![(row + 1, col + 1)];
        let side_positions = if random() > 0.5f64 {
            // cant use system dependant rand in wasm
            vec![left_positions, right_positions].concat()
        } else {
            vec![right_positions, left_positions].concat()
        };

        let empty_downwards_positions = self.find_valid_positions_for_solid(downwards_positions);
        let empty_side_positions = self.find_valid_positions_for_solid(side_positions);

        if let Some(down_pos) = empty_downwards_positions.last() {
            self.cells[idx].energy += 1; // When objects are falling they gain energy
            let new_idx = self.get_index(down_pos.0, down_pos.1);
            self.switch_cells(idx, new_idx);
        } else if let Some(side_pos) = empty_side_positions.last() {
            let new_idx = self.get_index(side_pos.0, side_pos.1);
            self.switch_cells(idx, new_idx);
        } else {
            self.cells[idx].energy = 0;
        }
    }

    fn update_water(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].has_been_updated = true;
        let cell_energy = self.cells[idx].energy;

        let downwards_positions: Vec<_> = (1..=cell_energy + 1).map(|i| (row + i, col)).collect();
        let left_down_positions = vec![(row + 1, col - 1)];
        let right_down_positions = vec![(row + 1, col + 1)];
        let left_positions: Vec<_> = (1..=SPREAD_FACTOR).map(|i| (row, col - i)).collect();
        let right_positions: Vec<_> = (1..=SPREAD_FACTOR).map(|i| (row, col + i)).collect();

        let side_down_positions = if random() > 0.5f64 {
            vec![left_down_positions, right_down_positions].concat()
        } else {
            vec![right_down_positions, left_down_positions].concat()
        };
        let side_positions = if random() > 0.5f64 {
            vec![left_positions, right_positions].concat()
        } else {
            vec![right_positions, left_positions].concat()
        };

        let empty_downwards_positions = self.find_valid_positions(downwards_positions);
        let empty_side_positions = self.find_valid_positions(side_positions);
        let empty_side_down_positions = self.find_valid_positions(side_down_positions);

        if let Some(down_pos) = empty_downwards_positions.last() {
            self.cells[idx].energy += 1; // When objects are falling they gain energy
            let new_idx = self.get_index(down_pos.0, down_pos.1);
            self.switch_cells(idx, new_idx);
        } else if let Some(side_down_pos) = empty_side_down_positions.last() {
            let new_idx = self.get_index(side_down_pos.0, side_down_pos.1);
            self.switch_cells(idx, new_idx);
        } else if let Some(side_pos) = empty_side_positions.last() {
            let new_idx = self.get_index(side_pos.0, side_pos.1);
            self.cells[idx].energy = 0;
            self.switch_cells(idx, new_idx);
        } else {
            self.cells[idx].energy = 0;
        }
    }

    fn update_rock(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].has_been_updated = true
    }
}


/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for row in (0..self.height).rev() {
            for col in (0..self.width).rev() {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                if cell.has_been_updated {
                    continue;
                }
                match cell.id {
                    CellType::Dead => (),
                    CellType::Sand => self.update_sand(row, col),
                    CellType::Water => self.update_water(row, col),
                    CellType::Rock => self.update_rock(row, col),
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

        let width = WIDTH;
        let height = HEIGHT;

        let cells = (0..width * height)
            .map(|_i| Cell::new(CellType::Dead))
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
        self.cells
            .iter()
            .map(|&c| c.id)
            .collect::<Vec<CellType>>()
            .as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height)
            .map(|_i| Cell::new(CellType::Dead))
            .collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height)
            .map(|_i| Cell::new(CellType::Dead))
            .collect();
    }

    pub fn set_cell(&mut self, row: u32, column: u32, ct: CellType) {
        // The out of bounds check is done in javascript
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
