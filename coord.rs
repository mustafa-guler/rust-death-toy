#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: f64,
    pub y: f64
}


impl Coord {
    pub fn new(x: f64, y: f64) -> Coord {
        Coord { x, y }
    }
}
