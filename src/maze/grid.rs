extern crate slog_async;
extern crate slog_term;

use super::cell::Cell;
use super::cell::Cells;
use slog::Drain;
use slog::Logger;
use std::collections::HashSet;

pub struct Grid {
    width: usize,
    length: usize,
    cells: Cells,
    visited: HashSet<(usize, usize)>,
}

impl Grid {
    pub fn new(width: usize, length: usize) -> Grid {
        Grid {
            width: width,
            length: length,
            cells: Vec::with_capacity(width * length),
            visited: HashSet::with_capacity(width * length),
        }
    }

    pub fn init(&mut self) {
        for row in 0..self.length() {
            for col in 0..self.width() {
                let cell = Cell::new(col, row);
                self.cells.push(cell);
            }
        }
    }

    pub fn display(&self) {
        let mut col = 1;
        let mut cells = self.cells();
        debug!(Grid::logger(), "{:?}", cells);
        cells.sort_by_key(|c| self.cells_len() - c.row());
        debug!(Grid::logger(), "{:?}", cells);
        for cell in cells {
            print!("{}|", cell.to_string());
            if col % self.width() == 0 {
                print!("\n");
            }
            col += 1;
        }
    }
    pub fn render_ascii(&self) {
        let mut output = "".to_string();
        let mut col = 1;
        let mut cells = self.cells();
        debug!(Grid::logger(), "{:?}", cells);
        cells.sort_by_key(|c| self.cells_len() - c.row());
        debug!(Grid::logger(), "{:?}", cells);

        output = format!("{}{}\n", "+", "---+".repeat(self.width()));

        let mut top = "|".to_string();
        let mut bottom = "+".to_string();

        for cell in cells {
            let body = "   ".to_string();
            let east_boundary = match cell.east {
                true => " ".to_string(),
                false => "|".to_string(),
            };
            top.push_str(&format!("{}{}", body, east_boundary));

            let south_boundary = match cell.south {
                true => "   ",
                false => "---",
            };
            let corner = "+";
            bottom.push_str(&format!("{}{}", south_boundary, corner));
            if col % self.width() == 0 {
                output.push_str(&format!("{}\n", top));
                output.push_str(&format!("{}\n", bottom));
                top = "|".to_string();
                bottom = "+".to_string();
            }
            col += 1;
        }
        print!("{}", output);
    }

    pub fn cell_at(&self, col: usize, row: usize) -> Option<Cell> {
        match (col, row) {
            (col, _) if col >= self.width() => None,
            (_, row) if row >= self.length() => None,
            (col, row) => {
                let index = (self.width() * row) + col;
                if index <= (self.width() * self.length()) - 1 {
                    Some(self.cells[index])
                } else {
                    None
                }
            }
        }
    }

    pub fn update_cell_at(&mut self, col: usize, row: usize, cell: Cell) {
        match (col, row) {
            (col, _) if col >= self.width() => panic!("Tried to update ({},{})", col, row),
            (_, row) if row >= self.length() => panic!("Tried to update ({},{})", col, row),
            (col, row) => {
                let index = (self.width() * row) + col;
                if index <= (self.width() * self.length()) - 1 {
                    self.cells[index] = cell
                } else {
                    panic!("Tried to update ({},{})", col, row)
                }
            }
        }
    }

    pub fn cells(&self) -> Cells {
        self.cells.clone()
    }

    pub fn cells_len(&self) -> usize {
        self.cells.len()
    }

    pub fn link_east(&mut self, col: usize, row: usize) {
        if let Some(mut cell) = self.cell_at(col, row) {
            cell.link_east();
            self.update_cell_at(col, row, cell);
        }
        if let Some(mut cell) = self.cell_at(col + 1, row) {
            cell.link_west();
            self.update_cell_at(col + 1, row, cell);
        }
    }

    pub fn link_north(&mut self, col: usize, row: usize) {
        if let Some(mut cell) = self.cell_at(col, row) {
            cell.link_north();
            self.update_cell_at(col, row, cell);
        }
        if let Some(mut cell) = self.cell_at(col, row + 1) {
            cell.link_south();
            self.update_cell_at(col, row + 1, cell);
        }
    }

    pub fn at_upper(&self, row: usize) -> bool {
        (self.length == row + 1)
    }

    pub fn at_lower(&self, row: usize) -> bool {
        (1 == row + 1)
    }

    pub fn at_right(&self, col: usize) -> bool {
        (self.width == col + 1)
    }

    pub fn at_left(&self, col: usize) -> bool {
        (1 == col + 1)
    }

    pub fn at_upper_left(&self, col: usize, row: usize) -> bool {
        self.at_upper(row) && self.at_left(col)
    }

    pub fn at_upper_right(&self, col: usize, row: usize) -> bool {
        self.at_upper(row) && self.at_right(col)
    }

    pub fn at_lower_left(&self, col: usize, row: usize) -> bool {
        self.at_lower(row) && self.at_left(col)
    }

    pub fn at_lower_right(&self, col: usize, row: usize) -> bool {
        self.at_lower(row) && self.at_right(col)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn length(&self) -> usize {
        self.length
    }

    fn logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let log = slog::Logger::root(drain, o!());

        log
    }
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 == 0, false);
    }

    #[test]
    fn new_grid() {
        let mut grid = Grid::new(4, 4);
        assert_eq!(grid.length(), 4);
        assert_eq!(grid.width(), 4);

        grid.init();
        assert_eq!(grid.cells_len(), 16);

        assert_eq!(grid.at_left(0), true);
        assert_eq!(grid.at_right(3), true);
        assert_eq!(grid.at_lower(0), true);
        assert_eq!(grid.at_upper(3), true);

        assert_eq!(grid.at_lower_left(0, 0), true);
        assert_eq!(grid.at_upper_left(0, 3), true);

        assert_eq!(grid.at_lower_right(3, 0), true);
        assert_eq!(grid.at_upper_right(3, 3), true);
    }

    #[test]
    fn cell_at_0_0() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(0, 0);
        assert_eq!(test, Some(Cell::new(0, 0)));
    }

    #[test]
    fn cell_at_1_1() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(1, 1);
        assert_eq!(test, Some(Cell::new(1, 1)));
    }

    #[test]
    fn cell_at_2_2() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(2, 2);
        assert_eq!(test, Some(Cell::new(2, 2)));
    }

    #[test]
    fn cell_at_1_0() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(1, 0);
        assert_eq!(test, Some(Cell::new(1, 0)));
    }

    #[test]
    fn cell_at_3_0() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(3, 0);
        assert_eq!(test, None);
        // assert_eq!(grid.cell_at(0, 0).east, false);
    }

    #[test]
    fn cell_at_0_1() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(0, 1);
        assert_eq!(test, Some(Cell::new(0, 1)));
    }

    #[test]
    fn cell_at_0_3() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(0, 3);
        assert_eq!(test, None);
    }

    #[test]
    fn link_east() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        grid.link_east(0, 0);

        let mut cell_0 = Cell::new(0, 0);
        let mut cell_1 = Cell::new(1, 0);
        cell_0.link_east();
        cell_1.link_west();

        let test_0 = grid.cell_at(0, 0);
        assert_eq!(test_0, Some(cell_0));

        let test_1 = grid.cell_at(1, 0);
        assert_eq!(test_1, Some(cell_1));
    }

    #[test]
    fn link_north() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        grid.link_north(0, 0);

        let mut cell_0 = Cell::new(0, 0);
        let mut cell_1 = Cell::new(0, 1);
        cell_0.link_north();
        cell_1.link_south();

        let test_0 = grid.cell_at(0, 0);
        assert_eq!(test_0, Some(cell_0));

        let test_1 = grid.cell_at(0, 1);
        assert_eq!(test_1, Some(cell_1));
    }
}
