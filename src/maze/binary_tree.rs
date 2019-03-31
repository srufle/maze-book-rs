use crate::maze::Direction;
use crate::maze::Maze;
use crate::maze::MazePosition;
pub fn generate(maze: &Maze) -> Vec<MazePosition> {
    let mut positions: Vec<MazePosition> = Vec::new();
    for row in 0..maze.length() {
        for col in 0..maze.width() {
            if maze.at_upper_right(col, row) {
                positions.push(MazePosition::new(col, row, Direction::None));
            } else if maze.at_upper(row) {
                positions.push(MazePosition::new(col, row, Direction::East));
            } else if maze.at_right(col) {
                positions.push(MazePosition::new(col, row, Direction::North));
            } else {
                let coin = crate::maze::coin_flip();
                if coin {
                    positions.push(MazePosition::new(col, row, Direction::North));
                } else {
                    positions.push(MazePosition::new(col, row, Direction::East));
                }
            }
        }
    }
    positions
}
