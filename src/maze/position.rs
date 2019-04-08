use super::direction::Direction;
use super::direction::Directions;
use std::cell::RefCell;
use std::fmt;

pub type Positions = RefCell<Vec<Position>>;

#[derive(Debug, Clone)]
pub struct Position {
    col: usize,
    row: usize,
    directions: Directions,
}

impl Position {
    pub fn new(col: usize, row: usize, directions: Direction) -> Position {
        let _directions: Directions = RefCell::new(Vec::new());
        _directions.borrow_mut().push(directions);
        Position {
            col: col,
            row: row,
            directions: _directions,
        }
    }
    pub fn col(&self) -> usize {
        self.col
    }
    pub fn row(&self) -> usize {
        self.row
    }
    pub fn push_direction(&self, direction: Direction) {
        self.directions.borrow_mut().push(direction);
    }

    pub fn pop_direction(&self) -> Option<Direction> {
        self.directions.borrow_mut().pop()
    }

    pub fn directions(&self) -> Vec<Direction> {
        self.directions.borrow_mut().to_vec()
    }
    pub fn direction(&self) -> Direction {
        let direction = self.directions.borrow_mut().to_vec().pop();
        match direction {
            Some(Direction::North) => Direction::North,
            Some(Direction::East) => Direction::East,
            Some(Direction::South) => Direction::South,
            Some(Direction::West) => Direction::West,
            _ => Direction::None,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let directions = self.directions();
        let mut items: Vec<String> = Vec::new();
        for dir in directions {
            let s = format!("{}", dir);
            items.push(s);
        }
        let direction_str = format!("({})", &items.join(","));
        write!(
            f,
            "{col:02}, {row:02}, {directions}",
            col = self.col,
            row = self.row,
            directions = direction_str
        )
    }
}
