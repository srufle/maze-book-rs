extern crate slog_async;
extern crate slog_term;

use slog::Drain;
use slog::Logger;
use std::cell::RefCell;

use super::direction::*;
use super::position::*;

#[derive(Debug)]
pub struct Maze {
    width: usize,
    length: usize,
    positions: Positions,
}

impl Maze {
    pub fn new(width: usize, length: usize) -> Maze {
        let positions: Positions = RefCell::new(Vec::new());
        Maze {
            width: width,
            length: length,
            positions: positions,
        }
    }
    pub fn blank(width: usize, length: usize) -> Maze {
        let maze = Maze::new(width, length);
        for row in 0..maze.length() {
            for col in 0..maze.width() {
                maze.push_position(Position::new(col, row, Direction::None));
            }
        }
        maze
    }
    fn logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let log = slog::Logger::root(drain, o!());

        log
    }
    pub fn total_cells(&self) -> usize {
        self.length * self.width
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

    pub fn display(&self) {
        let mut col = 1;
        let mut positions = self.positions();
        debug!(Maze::logger(), "{:?}", positions);
        positions.sort_by_key(|p| self.total_cells() - p.row());
        debug!(Maze::logger(), "{:?}", positions);
        for pos in positions {
            print!("{}|", pos.to_string());
            if col % self.width() == 0 {
                print!("\n");
            }
            col += 1;
        }
    }

    pub fn push_position(&self, position: Position) {
        self.positions.borrow_mut().push(position);
    }

    pub fn positions(&self) -> Vec<Position> {
        self.positions.borrow_mut().to_vec()
    }
}