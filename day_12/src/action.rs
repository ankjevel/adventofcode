#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

use Action::*;

impl Action {
    pub fn new(input: char) -> Self {
        match input {
            'N' => North,
            'S' => South,
            'E' => East,
            'W' => West,
            'L' => Left,
            'R' => Right,
            'F' => Forward,
            _ => North,
        }
    }
}
