use crate::maze::Grid;
use crate::maze::utils;

pub fn generate(maze: &mut Grid) {
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

pub fn generate_fixed_3x3_grid(maze: &mut Grid) {

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

pub fn generate_fixed_4x4_grid(maze: &mut Grid) {

// 00, 03, (E,S)|01, 03, (E,W)|02, 03, (E,W)  |03, 03, (S,W)|
// 00, 02, (N)  |01, 02, (E,S)|02, 02, (E,S,W)|03, 02, (N,S,W)|
// 00, 01, (E,S)|01, 01, (N,W)|02, 01, (N,S)  |03, 01, (N,S)|
// 00, 00, (N)  |01, 00, (E)  |02, 00, (N,W)  |03, 00, (N)|

// +---+---+---+---+
// |               |
// +   +---+---+   +
// |   |           |
// +---+   +   +   +
// |       |   |   |
// +   +---+   +   +
// |   |       |   |
// +---+---+---+---+

maze.link_north(0, 0);
maze.link_east(1, 0);
maze.link_north(2, 0);
maze.link_north(3, 0);

maze.link_east(0, 1);
maze.link_north(1, 1);
maze.link_north(2, 1);
maze.link_north(3, 1);

maze.link_north(0, 2);
maze.link_east(1, 2);
maze.link_east(2, 2);
maze.link_north(3, 2);

maze.link_east(0, 3);
maze.link_east(1, 3);
maze.link_east(2, 3);

}