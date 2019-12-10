#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl Point {
    pub fn from_usize(x: usize, y: usize) -> Point {
        Point {
            x: x as u64,
            y: y as u64,
        }
    }
}
