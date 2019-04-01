use crate::maze::Direction;
use crate::maze::Maze;
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
                let coin = crate::maze::coin_flip();
                if coin {
                    maze.push_position(Position::new(col, row, Direction::North));
                } else {
                    maze.push_position(Position::new(col, row, Direction::East));
                }
            }
        }
    }
}
