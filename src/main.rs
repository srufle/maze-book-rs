#[macro_use]
extern crate slog;

mod maze;
use maze::Grid;
use maze::Maze;
#[cfg(not(test))]
fn main() {
    let size = 4;
    println!("blank");
    let maze = Maze::blank(size, size);
    maze.display();

    println!("binary_tree grid");
    let mut grid = Grid::new(size, size);
    grid.init();
    maze::binary_tree::generate_grid(&mut grid);
    grid.display();
    grid.render_ascii();

    println!("binary_tree");
    let maze = Maze::new(size, size);
    maze::binary_tree::generate(&maze);
    maze.display();

    println!("sidewinder");
    let maze_sw = Maze::new(size, size);
    maze::sidewinder::generate(&maze_sw);
    maze_sw.display();

    println!("sidewinder grid");
    let mut grid = Grid::new(size, size);
    grid.init();
    maze::sidewinder::generate_grid(&mut grid);
    grid.display();
    grid.render_ascii();
}
