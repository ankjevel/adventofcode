#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

use Dir::*;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Direction(pub Dir);

impl Direction {
    pub fn new() -> Direction {
        Direction(Up)
    }

    pub fn turn(&mut self, turn: u8) {
        self.0 = match (self.0, turn) {
            (Up, 0) | (Down, 1) => Left,
            (Right, 0) | (Left, 1) => Up,
            (Down, 0) | (Up, 1) => Right,
            (Left, 0) | (Right, 1) => Down,
            _ => panic!("turn not covered"),
        }
    }
}
