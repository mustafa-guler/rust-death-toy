pub mod cell;
pub mod coord;
pub mod dists;
pub mod kdtree;

pub use crate::cell::Cell;
pub use crate::coord::Coord;

// const TISSUE_RADIUS: f64 = 4000.0;
const CELL_RADIUS: f64 = 10.0;

const VEL_MAX: f64 = 20.0 / 60.0;
const VEL_MIN: f64 = 0.0;
const IM_MAX: i64 = 40 * 60;
const IM_MIN: i64 = 10 * 60;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
