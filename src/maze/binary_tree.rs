use crate::maze::Grid;
use crate::maze::Maze;
use crate::maze::utils;
use crate::maze::Direction;
use crate::maze::Position;

pub fn generate(maze: &Maze) {
    for row in 0..maze.length() {
        for col in 0..maze.width() {
            if maze.at_upper_right(col, row) {
                maze.push_position(Position::new(col, row, Direction::None));
            } else if maze.at_upper(row) {
                maze.push_position(Position::new(col, row, Direction::East));
            } else if maze.at_right(col) {
                maze.push_position(Position::new(col, row, Direction::North));
            } else {
                let coin = utils::coin_flip();
                if coin {
                    maze.push_position(Position::new(col, row, Direction::North));
                } else {
                    maze.push_position(Position::new(col, row, Direction::East));
                }
            }
        }
    }
}

pub fn generate_grid(maze: &mut Grid) {
    for row in 0..maze.length() {
        for col in 0..maze.width() {
            if maze.at_upper_right(col, row) {
                // nothing to do. just leave alone
            } else if maze.at_upper(row) {
                maze.link_east(col, row);
            } else if maze.at_right(col) {
                maze.link_north(col, row);
            } else {
                let coin = utils::coin_flip();
                if coin {
                    maze.link_north(col, row);
                } else {
                    maze.link_east(col, row);
                }
            }
        }
    }
}

pub fn generate_fixed_grid(maze: &mut Grid) {

// 00, 02, (E)  |01, 02, (E,W)  |02, 02, (S,W)|
// 00, 01, (E,S)|01, 01, (E,S,W)|02, 01, (N,S,W)|
// 00, 00, (N)  |01, 00, (N)    |02, 00, (N)|

// +---+---+---+
// |           |
// +---+---+   +
// |           |
// +   +   +   +
// |   |   |   |
// +---+---+---+
maze.link_north(0, 0);
maze.link_north(1, 0);
maze.link_north(2, 0);

maze.link_east(0, 1);
maze.link_east(1, 1);
maze.link_north(2, 1);

maze.link_east(0, 2);
maze.link_east(1, 2);

}