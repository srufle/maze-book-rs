extern crate slog_async;
extern crate slog_term;

use std::fmt;

pub type Cells = Vec<Cell>;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Cell {
    col: usize,
    row: usize,
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

impl Cell {
    pub fn new(col: usize, row: usize) -> Cell {
        Cell {
            col: col,
            row: row,
            north: false,
            east: false,
            south: false,
            west: false,
        }
    }

    pub fn link_north(&mut self) {
        self.north = true
    }
    pub fn link_east(&mut self) {
        self.east = true
    }
    pub fn link_south(&mut self) {
        self.south = true
    }
    pub fn link_west(&mut self) {
        self.west = true
    }

    pub fn row(&self) -> usize {
        self.row
    }
    pub fn col(&self) -> usize {
        self.col
    }
    pub fn row_i32(&self) -> i32 {
        (self.row as i32)
    }
    pub fn col_i32(&self) -> i32 {
        (self.col as i32)
    }

    pub fn inner_x(&self, margin: i32, cell_size: i32, line_width: u32) -> i32 {
        let lw_i32 = line_width as i32;
        (margin + (self.col_i32() * cell_size) + lw_i32)
    }
    pub fn inner_y(&self, margin: i32, cell_size: i32, line_width: u32) -> i32 {
        let lw_i32 = line_width as i32;
        (margin - (self.row_i32() * cell_size) - cell_size + lw_i32)
    }
    pub fn inner_w(&self, cell_size: i32, line_width: u32) -> u32 {
        let lw_i32 = line_width as i32;
        (cell_size - lw_i32) as u32
    }
    pub fn inner_h(&self, cell_size: i32, line_width: u32) -> u32 {
        let lw_i32 = line_width as i32;
        (cell_size - lw_i32) as u32
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items: Vec<String> = Vec::new();
        if self.north {
            items.push("N".to_string());
        }
        if self.east {
            items.push("E".to_string());
        }
        if self.south {
            items.push("S".to_string());
        }
        if self.west {
            items.push("W".to_string());
        }

        let direction_str = format!("({})", &items.join(","));

        write!(
            f,
            "{col:02}, {row:02}, {directions}",
            col = self.col,
            row = self.row,
            directions = direction_str
        )
    }
}
