#[macro_use]
extern crate slog;

mod maze;
use maze::Grid;
use maze::Maze;
#[cfg(not(test))]
fn main() {
    println!("blank");
    let maze = Maze::blank(4, 4);
    maze.display();

    println!("binary_tree grid");
    let mut grid = Grid::new(4, 4);
    grid.init();
    maze::binary_tree::generate_grid(&mut grid);
    grid.display();

    println!("binary_tree");
    let maze = Maze::new(4, 4);
    maze::binary_tree::generate(&maze);
    maze.display();

    println!("sidewinder");
    let maze_sw = Maze::new(4, 4);
    maze::sidewinder::generate(&maze_sw);
    maze_sw.display();

    println!("sidewinder grid");
    let mut grid = Grid::new(4, 4);
    grid.init();
    maze::sidewinder::generate_grid(&mut grid);
    grid.display();
}
