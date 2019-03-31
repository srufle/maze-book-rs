use crate::maze::Direction;
use crate::maze::Maze;
use crate::maze::Position;
pub fn generate(maze: &Maze) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    for row in 0..maze.length() {
        for col in 0..maze.width() {
            if maze.at_upper_right(col, row) {
                positions.push(Position::new(col, row, Direction::None));
            } else if maze.at_upper(row) {
                positions.push(Position::new(col, row, Direction::East));
            } else if maze.at_right(col) {
                positions.push(Position::new(col, row, Direction::North));
            } else {
                let coin = crate::maze::coin_flip();
                if coin {
                    positions.push(Position::new(col, row, Direction::North));
                } else {
                    positions.push(Position::new(col, row, Direction::East));
                }
            }
        }
    }
    positions
}
