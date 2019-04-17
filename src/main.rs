#[macro_use]
extern crate slog;

mod maze;
use maze::Distances;
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
    // maze::binary_tree::generate_fixed_4x4_grid(&mut grid);
    grid.display();
    grid.render_ascii();
    let mut distances = Distances::new(&grid);
    let mut frontier: Vec<(usize, usize)> = vec![];
    frontier.push((0, 0));
    grid.visit_cell((0, 0));
    distances.calculate_distances(&mut grid, frontier);
    grid.render_png(&"./binary_tree_grid.png".to_string());

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
