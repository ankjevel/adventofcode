use std::cmp::Ordering::{self, Equal, Greater, Less};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.y == other.y {
            if self.x < other.x {
                Less
            } else if self.x > other.x {
                Greater
            } else {
                Equal
            }
        } else if self.y < other.y {
            Less
        } else if self.y > other.y {
            Greater
        } else {
            if self.x < other.x {
                Less
            } else if self.x > other.x {
                Greater
            } else {
                Equal
            }
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
