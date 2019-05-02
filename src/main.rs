#[macro_use]
extern crate slog;

mod maze;
use maze::Grid;
use maze::Pos2d;
#[cfg(not(test))]
fn main() {
    let size = 2;

    println!("aldous_broder grid");
    let mut grid = Grid::new(size, size);
    grid.init();
    // grid.entrance(Pos2d::p(0, 0));
    maze::aldous_broder::generate(&mut grid);
    grid.display();
    grid.calculate_distances();
    grid.render_ascii();

    println!("Binary Tree grid");
    let mut grid = Grid::new(size, size);
    grid.init();
    grid.entrance(Pos2d::p(0, 0));
    maze::binary_tree::generate(&mut grid);
    // maze::binary_tree::generate_fixed_4x4_grid(&mut grid);
    // maze::binary_tree::generate_fixed_3x3_grid(&mut grid);

    println!("Display internal structure");
    grid.display();

    println!("Calculating distances");
    grid.calculate_distances();

    println!("Render as ASCII");
    grid.render_ascii();

    println!("Plot shortest path");
    let dist_map = grid.plot_path_between(Pos2d::p(0, 0), Pos2d::p(size - 1, size - 1));
    grid.render_ascii_path(dist_map);

    println!("Calculating longest path");
    let max_path1 = grid.max_path_from(Pos2d::p(0, 0));
    let max_path2 = grid.max_path_from(max_path1.pos());

    println!("max path 1 = {:?}, max path 2 = {:?}", max_path1, max_path2);

    println!("Plot longest path");
    let dist_map = grid.plot_path_between(Pos2d::p(0, 0), max_path2.pos());
    grid.render_ascii_path(dist_map);

    println!("Display Distances");
    grid.display_distances();

    println!("Render PNG");
    grid.render_png(&"./binary_tree.png".to_string());

    println!("Render Path PNG");
    let dist_map = grid.plot_path_between(Pos2d::p(0, 0), max_path2.pos());
    grid.render_png_path(&"./binary_tree_path.png".to_string(), dist_map);

    println!("sidewinder grid");
    let mut grid = Grid::new(size, size);
    grid.init();
    grid.entrance(Pos2d::p(0, 0));
    maze::sidewinder::generate(&mut grid);
    grid.display();
    grid.calculate_distances();
    grid.render_ascii();
}
