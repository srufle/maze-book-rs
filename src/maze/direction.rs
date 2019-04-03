use std::cell::RefCell;
use std::fmt;
pub type Directions = RefCell<Vec<Direction>>;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    None,
    North,
    East,
    South,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Direction::None => "0",
            Direction::North => "N",
            Direction::East => "E",
            Direction::South => "S",
            Direction::West => "W",
        };
        write!(f, "{}", printable)
    }
}