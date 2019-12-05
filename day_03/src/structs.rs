#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Movement {
    pub direction: Direction,
    pub steps: i32,
}

impl Movement {
    pub fn new(input: &str) -> Movement {
        let mut chars = input.chars();

        let direction = match chars.next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("direction not defined"),
        };

        let steps = chars.as_str().parse::<i32>().unwrap();

        Movement { direction, steps }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: i32, // L-R
    pub y: i32, // U-D
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}
