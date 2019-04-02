#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate rand;

use slog::Drain;
use slog::Logger;

mod maze;
use maze::Maze;

fn main() {
    
    println!("binary_tree");
    let maze = Maze::new(4, 4);
    maze::binary_tree::generate(&maze);
    maze.display();

    println!("sidewinder");
    let maze_sw = Maze::new(4, 4);
    maze::sidewinder::generate(&maze_sw);
    maze_sw.display();
    
}

fn logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    log
}

