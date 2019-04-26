use crate::maze::utils;
use crate::maze::Grid;

pub fn generate(maze: &mut Grid) {
    for row in 0..maze.length() {
        // May use visited
        let mut run: Vec<(usize, usize)> = Vec::new();
        for col in 0..maze.width() {
            if maze.at_upper_right(col, row) {
                run.push((col, row));
            } else if maze.at_upper(row) {
                maze.link_east(col, row);
                run.push((col, row));
            } else if maze.at_right(col) {
                maze.link_north(col, row);
                run.push((col, row));
            } else {
                let coin = utils::coin_flip();
                if coin {
                    if run.len() == 0 {
                        maze.link_north(col, row);
                        run.push((col, row));
                    } else if run.len() >= 1 {
                        maze.link_east(col, row);
                        run.push((col, row));
                        let low = 0;
                        let high = run.len() - 1;
                        let run_index: usize = utils::choose_cell(low, high) as usize;
                        maze.link_north(run_index, row);
                    }

                    run.clear();
                } else {
                    maze.link_east(col, row);
                    run.push((col, row));
                }
            }
        }
    }
}
