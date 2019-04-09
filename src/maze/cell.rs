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
