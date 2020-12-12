use crate::action::Action::{self, *};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Facing {
    East,
    West,
    North,
    South,
}

impl Facing {
    pub fn from(action: Action) -> Self {
        match action {
            North => Self::North,
            West => Self::West,
            South => Self::South,
            East | _ => Self::East,
        }
    }
}
