extern crate rand;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use rand::prelude::*;
use slog::Drain;
use slog::Logger;

mod maze {
    use slog::Drain;
    use slog::Logger;

    #[derive(Debug)]
    pub struct Maze {
        width: u32,
        length: u32,
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

        pub fn grid(&self) -> Vec<u32> {
            let size = self.total_cells();
            debug!(Maze::logger(), "size={}", size);
            let vec = vec![0; size as usize];
            vec
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
    }

    #[derive(Debug)]
    pub enum Direction {
        None,
        North,
        East,
        South,
        West,
    }
}
fn logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    log
}
fn coin_flip() -> bool {
    let mut rng = StdRng::from_entropy();
    rng.gen_bool(0.5)
}

use maze::Direction;
use maze::Maze;

fn main() {
    let mut flips: Vec<(u32, u32, Direction)> = Vec::new();
    let maze = Maze::new(4, 4);
    let upper_limit = maze.total_cells();
    let our_grid = maze.grid();

    info!(logger(), "{:?}", our_grid);

    for row in 0..maze.length() {
        for col in 0..maze.width() {
            if maze.at_upper_right(col, row) {
                flips.push((col, row, Direction::None));
            } else if maze.at_upper(row) {
                flips.push((col, row, Direction::East));
            } else if maze.at_right(col) {
                flips.push((col, row, Direction::North));
            } else {
                let coin = coin_flip();
                if coin {
                    flips.push((col, row, Direction::North));
                } else {
                    flips.push((col, row, Direction::East));
                }
            }
        }
    }
    info!(logger(), "Maze = {:?} ", flips);
}
