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

    println!("Binary Tree grid");
    let mut grid = Grid::new(size, size);
    grid.init();
    grid.entrance((0, 0));
    maze::binary_tree::generate_grid(&mut grid);
    // maze::binary_tree::generate_fixed_4x4_grid(&mut grid);

    println!("Display internal structure");
    grid.display();

    println!("Calculating distances");
    grid.calculate_distances();

    println!("Render as ASCII");
    grid.render_ascii();

    println!("Plot shortest path");
    grid.plot_path_between((0, 0), (size - 1, size - 1));

    println!("Calculating longest path");
    let max_path1 = grid.max_path_from((0, 0));
    let max_path2 = grid.max_path_from(max_path1.0);

    println!("max path 1 = {:?}, max path 2 = {:?}", max_path1, max_path2);

    println!("Plot longest path");
    grid.plot_path_between((0, 0), max_path2.0);
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
