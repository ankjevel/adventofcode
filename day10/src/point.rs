use crate::Input;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: i128,
    pub y: i128,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Velocity {
    pub x: i128,
    pub y: i128,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub position: Position,
    pub velocity: Velocity,
}

impl Point {
    pub fn new(input: Input) -> Self {
        Self {
            position: Position {
                x: input.0,
                y: input.1,
            },
            velocity: Velocity {
                x: input.2,
                y: input.3,
            },
        }
    }

    pub fn new_using_position(x: isize, y: isize) -> Self {
        Self {
            position: Position {
                x: x as i128,
                y: y as i128,
            },
            ..Point::default()
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position)
    }
}
