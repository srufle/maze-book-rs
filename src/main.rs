#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate rand;

use rand::prelude::*;
use slog::Drain;
use slog::Logger;

mod maze;
use maze::Direction;
use maze::Maze;
use maze::MazePosition as MazPos;

fn main() {
    let mut maze_positions: Vec<MazPos> = Vec::new();
    let maze = Maze::new(4, 4);
    // maze.positions();
    // TODO: Move to seperate module files
    // Logic is for a Binary Tree
    // How can I abstract the how
    for row in 0..maze.length() {
        for col in 0..maze.width() {
            if maze.at_upper_right(col, row) {
                maze_positions.push(MazPos::new(col, row, Direction::None));
            } else if maze.at_upper(row) {
                maze_positions.push(MazPos::new(col, row, Direction::East));
            } else if maze.at_right(col) {
                maze_positions.push(MazPos::new(col, row, Direction::North));
            } else {
                let coin = coin_flip();
                if coin {
                    maze_positions.push(MazPos::new(col, row, Direction::North));
                } else {
                    maze_positions.push(MazPos::new(col, row, Direction::East));
                }
            }
        }
    }
    display_maze(maze, maze_positions);
}

fn display_maze(maze: Maze, mut positions: Vec<MazPos>) {
    let mut col = 1;

    debug!(logger(), "{:?}", positions);
    positions.sort_by_key(|p| maze.total_cells() - p.row());
    debug!(logger(), "{:?}", positions);
    for pos in positions {
        print!("{}|", pos.to_string());
        if col % maze.width() == 0 {
            print!("\n");
        }
        col += 1;
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
