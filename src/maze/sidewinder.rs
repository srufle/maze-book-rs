use crate::maze::utils;
use crate::maze::Direction;
use crate::maze::Grid;
use crate::maze::Maze;
use crate::maze::Position;

pub fn generate(maze: &Maze) {
    for row in 0..maze.length() {
        let mut run: Vec<Position> = Vec::new();
        for col in 0..maze.width() {
            if maze.at_upper_right(col, row) {
                run.push(Position::new(col, row, Direction::None));
            } else if maze.at_upper(row) {
                run.push(Position::new(col, row, Direction::East));
            } else if maze.at_right(col) {
                run.push(Position::new(col, row, Direction::North));
            } else {
                let coin = utils::coin_flip();
                if coin {
                    if run.len() == 0 {
                        run.push(Position::new(col, row, Direction::North));
                    } else if run.len() >= 1 {
                        run.push(Position::new(col, row, Direction::East));
                        let low = 0;
                        let high = run.len() - 1;
                        let run_index: usize = utils::choose_cell(low, high) as usize;
                        run[run_index].push_direction(Direction::North);
                    }

                    copy_run(maze, &run);
                    run.clear();
                } else {
                    run.push(Position::new(col, row, Direction::East));
                }
            }
        }

        copy_run(maze, &run);
    }
}

pub fn copy_run(maze: &Maze, run: &Vec<Position>) {
    if run.len() >= 1 {
        for p in run {
            let c = p.col();
            let r = p.row();
            let directions = p.directions();
            let new_pos = Position::new(c, r, Direction::None);
            let _olddir = new_pos.pop_direction();
            for d in directions {
                new_pos.push_direction(d);
            }

            maze.push_position(new_pos);
        }
    }
}

pub fn generate_grid(maze: &mut Grid) {
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
