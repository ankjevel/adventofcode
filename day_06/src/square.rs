use crate::Pos;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Square {
    Floor,
    Obstruction,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Facing {
    Up,
    Right,
    Down,
    Left,
}

use Facing::*;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Guard {
    pub position: (Pos, Pos),
    pub facing: Facing,
}

impl Guard {
    pub fn new(position: (Pos, Pos)) -> Self {
        Self {
            position,
            facing: Up,
        }
    }

    pub fn current(&self) -> ((Pos, Pos), Facing) {
        (self.position, self.facing)
    }

    pub fn set_position(&mut self, position: (Pos, Pos)) {
        self.position = position;
    }

    pub fn turn(&mut self) {
        self.facing = match self.facing {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
    }

    pub fn next(&self) -> (Pos, Pos) {
        let (x, y) = self.position;
        match self.facing {
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        }
    }
}
