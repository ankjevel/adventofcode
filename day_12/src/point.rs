#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new() -> Self {
        Self { ..Point::default() }
    }
}
