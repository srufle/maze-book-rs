extern crate slog;
extern crate slog_async;
extern crate slog_term;
use crate::maze::utils;
use crate::maze::Direction;
use crate::maze::Distance2d;
use crate::maze::Grid;
use crate::maze::Pos2d;
use crate::maze::Pos2dVec;

use slog::Drain;
use slog::Logger;

pub fn generate(maze: &mut Grid) {
    let mut unvisted_count: usize = maze.length() * maze.width() - 1;
    let col_low = 0;
    let col_high = maze.width() - 1;
    let col: usize = utils::choose_cell(col_low, col_high) as usize;

    let row_low = 0;
    let row_high = maze.length() - 1;
    let row: usize = utils::choose_cell(row_low, row_high) as usize;

    let mut cur_pos = Pos2d::p(col, row);
    while unvisted_count > 0 {
        if !maze.is_cell_visited(cur_pos) {
            debug!(
                logger(),
                "Start newly visited={:?}, unvisted_count={}", cur_pos, unvisted_count
            );
            maze.visit_pos(cur_pos);
            unvisted_count -= 1;
            let direction = utils::choose_direction();
            cur_pos = match direction {
                Direction::North => {
                    debug!(logger(), "Going {:?}", direction);
                    maze.link_north(cur_pos.col(), cur_pos.row());
                    maze.next_north(cur_pos.col(), cur_pos.row())
                }
                Direction::East => {
                    debug!(logger(), "Going {:?}", direction);
                    maze.link_east(cur_pos.col(), cur_pos.row());
                    maze.next_east(cur_pos.col(), cur_pos.row())
                }
                Direction::South => {
                    debug!(logger(), "Going {:?}", direction);
                    maze.link_south(cur_pos.col(), cur_pos.row());
                    maze.next_south(cur_pos.col(), cur_pos.row())
                }
                Direction::West => {
                    debug!(logger(), "Going {:?}", direction);
                    maze.link_west(cur_pos.col(), cur_pos.row());
                    maze.next_west(cur_pos.col(), cur_pos.row())
                }
            };
            debug!(
                logger(),
                "End newly visited={:?}, unvisted_count={}", cur_pos, unvisted_count
            );
        } else {
            debug!(
                logger(),
                "Start previously visited={:?}, unvisted_count={}", cur_pos, unvisted_count
            );
            let direction = utils::choose_direction();
            cur_pos = match direction {
                Direction::North => {
                    debug!(logger(), "Going {:?}", direction);
                    maze.next_north(cur_pos.col(), cur_pos.row())
                }
                Direction::East => {
                    debug!(logger(), "Going {:?}", direction);
                    maze.next_east(cur_pos.col(), cur_pos.row())
                }
                Direction::South => {
                    debug!(logger(), "Going {:?}", direction);
                    maze.next_south(cur_pos.col(), cur_pos.row())
                }
                Direction::West => {
                    debug!(logger(), "Going {:?}", direction);
                    maze.next_west(cur_pos.col(), cur_pos.row())
                }
            };
            debug!(
                logger(),
                "End previously visited={:?}, unvisted_count={}", cur_pos, unvisted_count
            );
        }
    }
}

fn logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!())
}
