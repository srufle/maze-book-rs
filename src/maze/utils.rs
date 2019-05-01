extern crate rand;
use crate::maze::Direction;
use rand::prelude::*;

pub fn coin_flip() -> bool {
    let mut rng = StdRng::from_entropy();
    rng.gen_bool(0.5)
}

pub fn choose_cell(low: usize, high: usize) -> usize {
    let mut rng = StdRng::from_entropy();
    rng.gen_range(low, high)
}

pub fn choose_direction_nesw() -> Direction {
    let mut rng = StdRng::from_entropy();
    Direction::from(rng.gen_range(0, 4))
}

pub fn choose_direction_ne() -> Direction {
    let mut rng = StdRng::from_entropy();
    Direction::from(rng.gen_range(0, 2))
}
