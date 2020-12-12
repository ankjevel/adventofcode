use crate::action::Action;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl Direction {
    pub fn turn(&mut self, action: &Action) {
        *self = match action {
            &Action::Left => match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            },
            &Action::Right | _ => match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            },
        };
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}
