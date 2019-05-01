extern crate slog;
extern crate slog_async;
extern crate slog_term;
use crate::maze::utils;
use crate::maze::Direction;
use crate::maze::Grid;
use crate::maze::Pos2d;

use slog::Drain;
use slog::Logger;

pub fn generate(maze: &mut Grid) {
    let mut unvisted_count: usize = maze.length() * maze.width() - 1;
    let col_low = 0;
    let col_high = maze.width();

    let row_low = 0;
    let row_high = maze.length();

    let col: usize = utils::choose_cell(col_low, col_high) as usize;
    let row: usize = utils::choose_cell(row_low, row_high) as usize;

    let mut cur_pos = Pos2d::p(col, row);
    let mut step = 0;
    while unvisted_count > 0 {
        step += 1;
        if !maze.is_cell_visited(cur_pos) {
            debug!(
                logger(),
                "{} - ------> newly visited={:?}, unvisted_count={}", step, cur_pos, unvisted_count
            );

            // let direction = utils::choose_direction_ne();
            let direction = utils::choose_direction_nesw();
            let next_pos = match direction {
                Direction::North => {
                    debug!(logger(), "{} - Move {:?}", step, direction);
                    // maze.link_north(cur_pos.col(), cur_pos.row());
                    maze.next_north(cur_pos.col(), cur_pos.row())
                }
                Direction::East => {
                    debug!(logger(), "{} - Move {:?}", step, direction);
                    // maze.link_east(cur_pos.col(), cur_pos.row());
                    maze.next_east(cur_pos.col(), cur_pos.row())
                }
                Direction::South => {
                    debug!(logger(), "{} - Move {:?}", step, direction);
                    // maze.link_south(cur_pos.col(), cur_pos.row());
                    maze.next_south(cur_pos.col(), cur_pos.row())
                }
                Direction::West => {
                    debug!(logger(), "{} - Move {:?}", step, direction);
                    // maze.link_west(cur_pos.col(), cur_pos.row());
                    maze.next_west(cur_pos.col(), cur_pos.row())
                }
                // Direction:: | Direction:: => panic!(),
            };
            if !maze.is_cell_visited(next_pos) {
                match direction {
                    Direction::North => {
                        debug!(logger(), "{} - Link {:?}", step, direction);
                        maze.link_north(cur_pos.col(), cur_pos.row());
                        // maze.next_north(cur_pos.col(), cur_pos.row())
                    }
                    Direction::East => {
                        debug!(logger(), "{} - Link {:?}", step, direction);
                        maze.link_east(cur_pos.col(), cur_pos.row());
                        // maze.next_east(cur_pos.col(), cur_pos.row())
                    }
                    Direction::South => {
                        debug!(logger(), "{} - Link {:?}", step, direction);
                        maze.link_south(cur_pos.col(), cur_pos.row());
                        // maze.next_south(cur_pos.col(), cur_pos.row())
                    }
                    Direction::West => {
                        debug!(logger(), "{} - Link {:?}", step, direction);
                        maze.link_west(cur_pos.col(), cur_pos.row());
                        // maze.next_west(cur_pos.col(), cur_pos.row())
                    }
                };
            }
            if next_pos != cur_pos {
                maze.visit_pos(cur_pos);
                // maze.visit_pos(next_pos);
                unvisted_count -= 1;
                cur_pos = next_pos;
            } 
            // else {
            //     let col: usize = utils::choose_cell(col_low, col_high) as usize;
            //     let row: usize = utils::choose_cell(row_low, row_high) as usize;

            //     // cur_pos = Pos2d::p(col, row);
            // }

            debug!(
                logger(),
                "{} - <------ newly visited={:?}, unvisted_count={}", step, cur_pos, unvisted_count,
            );
        } else {
            debug!(
                logger(),
                "{} - - - - - - -> previously visited={:?}, unvisted_count={}",
                step,
                cur_pos,
                unvisted_count
            );
            // let direction = utils::choose_direction_ne();
            let direction = utils::choose_direction_nesw();

            let next_pos = match direction {
                Direction::North => {
                    debug!(logger(), "{} - - - - - - -> Move {:?}", step, direction);
                    maze.next_north(cur_pos.col(), cur_pos.row())
                }
                Direction::East => {
                    debug!(logger(), "{} - - - - - - -> Move {:?}", step, direction);
                    maze.next_east(cur_pos.col(), cur_pos.row())
                }
                Direction::South => {
                    debug!(logger(), "{} - - - - - - -> Move {:?}", step, direction);
                    maze.next_south(cur_pos.col(), cur_pos.row())
                }
                Direction::West => {
                    debug!(logger(), "{} - - - - - - -> Move {:?}", step, direction);
                    maze.next_west(cur_pos.col(), cur_pos.row())
                }                
                // Direction::South | Direction::West => panic!(),
            };
            if next_pos != cur_pos {
                cur_pos = next_pos;
            }

            debug!(
                logger(),
                "{} - <- - - - - - - previously visited={:?}, unvisted_count={}",
                step,
                cur_pos,
                unvisted_count
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
