
extern crate rand;
use rand::prelude::*;

pub fn coin_flip() -> bool {
    let mut rng = StdRng::from_entropy();
    rng.gen_bool(0.5)
}

pub fn choose_cell(low: usize, high: usize) -> usize {
    let mut rng = StdRng::from_entropy();
    rng.gen_range(low, high)
}
