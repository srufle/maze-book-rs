extern crate rand;
extern crate slog_async;
extern crate slog_term;

use rand::prelude::*;
use slog::Drain;
use slog::Logger;
use std::fmt;
pub mod binary_tree;

#[derive(Debug)]
pub struct Maze {
    width: u32,
    length: u32,
}

pub fn coin_flip() -> bool {
    let mut rng = StdRng::from_entropy();
    rng.gen_bool(0.5)
}

impl Maze {
    pub fn new(width: u32, length: u32) -> Maze {
        Maze {
            width: width,
            length: length,
        }
    }
    fn logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let log = slog::Logger::root(drain, o!());

        log
    }
    pub fn total_cells(&self) -> u32 {
        self.length * self.width
    }

    pub fn at_upper(&self, row: u32) -> bool {
        (self.length == row + 1)
    }

    pub fn at_right(&self, col: u32) -> bool {
        (self.width == col + 1)
    }

    pub fn at_upper_right(&self, col: u32, row: u32) -> bool {
        self.at_upper(row) && self.at_right(col)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn display(&self, mut positions: Vec<MazePosition>) {
        let mut col = 1;

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
}

#[derive(Debug, Copy, Clone)]
pub struct MazePosition {
    col: u32,
    row: u32,
    direction: Direction,
}
impl MazePosition {
    pub fn new(col: u32, row: u32, direction: Direction) -> MazePosition {
        MazePosition {
            col: col,
            row: row,
            direction: direction,
        }
    }
    pub fn col(&self) -> u32 {
        self.col
    }
    pub fn row(&self) -> u32 {
        self.row
    }
}
impl fmt::Display for MazePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{col:02}, {row:02}, {direction}",
            col = self.col,
            row = self.row,
            direction = self.direction
        )
    }
}
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    None,
    North,
    East,
    South,
    West,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Direction::None => "0",
            Direction::North => "N",
            Direction::East => "E",
            Direction::South => "S",
            Direction::West => "W",
        };
        write!(f, "{}", printable)
    }
}
