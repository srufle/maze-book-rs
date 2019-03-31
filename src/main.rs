#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate rand;

// use rand::prelude::*;
use slog::Drain;
use slog::Logger;

mod maze;
// use maze::Direction;
use maze::Maze;
// use maze::MazePosition as MazPos;

fn main() {
    
    let maze = Maze::new(4, 4);
    let positions = maze.generate();
    maze.display(positions);
    // maze.positions();
    // Logic is for a Binary Tree
    // How can I abstract the how

    // display_maze(maze, positions);
}

// fn display_maze(maze: Maze, mut positions: Vec<MazPos>) {
//     let mut col = 1;

//     debug!(logger(), "{:?}", positions);
//     positions.sort_by_key(|p| maze.total_cells() - p.row());
//     debug!(logger(), "{:?}", positions);
//     for pos in positions {
//         print!("{}|", pos.to_string());
//         if col % maze.width() == 0 {
//             print!("\n");
//         }
//         col += 1;
//     }
// }
fn logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    log
}

