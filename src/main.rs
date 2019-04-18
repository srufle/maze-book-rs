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
    grid.entrance((0, 0));
    maze::binary_tree::generate_grid(&mut grid);
    // maze::binary_tree::generate_fixed_4x4_grid(&mut grid);
    grid.display();
    grid.calculate_distances();
    grid.render_ascii();

    grid.display_distances();
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
    grid.entrance((0, 0));
    maze::sidewinder::generate_grid(&mut grid);
    grid.display();
    grid.calculate_distances();
    grid.render_ascii();
}
