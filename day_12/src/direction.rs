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
    pub fn turn(&mut self, action: &Action, turns: u16) -> Direction {
        for _ in 0..turns {
            *self = match action {
                Action::Left => match self {
                    Up => Left,
                    Left => Down,
                    Down => Right,
                    Right => Up,
                },
                Action::Right | _ => match self {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                },
            };
        }
        self.clone()
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}
