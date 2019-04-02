use crate::maze::Direction;
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
                let coin = crate::maze::coin_flip();
                if coin {
                    if run.len() == 0 {
                        run.push(Position::new(col, row, Direction::North));
                    } else if run.len() >= 1 {
                        run.push(Position::new(col, row, Direction::East));
                        let low = &run[0];
                        let high = &run[run.len() - 1];
                        let run_index: usize =
                            crate::maze::choose_cell(low.col(), high.col()) as usize;
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