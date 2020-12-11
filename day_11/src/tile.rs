#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Floor,
    Empty,
    Occupied,
}

use Tile::*;

impl Tile {
    pub fn new(input: &char) -> Self {
        match input {
            '#' => Occupied,
            'L' => Empty,
            _ => Floor,
        }
    }
}
