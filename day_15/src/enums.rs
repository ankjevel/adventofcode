#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

use Direction::{Down, Left, Right, Up};

impl Direction {
    pub fn new(input: i64) -> Direction {
        match input {
            1 => Up,
            2 => Down,
            3 => Left,
            _ => Right,
        }
    }

    pub fn dir(input: Direction) -> i64 {
        match input {
            Up => 1,
            Down => 2,
            Left => 3,
            Right => 4,
        }
    }

    pub fn turn(self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Status {
    Wall = 0,
    Moved = 1,
    MovedAndOnOxygen = 2,
}

use Status::{Moved, MovedAndOnOxygen, Wall};

impl Status {
    pub fn new(input: i64) -> Status {
        match input {
            0 => Wall,
            1 => Moved,
            2 => MovedAndOnOxygen,
            _ => panic!("{} not known", input),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Tile {
    Current,
    Unknown,
    Wall,
    Visited,
    Oxygen,
}

impl Tile {
    pub fn new(input: &str) -> Tile {
        match input {
            "0" => Tile::Wall,
            "1" => Tile::Visited,
            _ => Tile::Oxygen,
        }
    }
}
