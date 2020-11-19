use regex::Regex;
use std::str::FromStr;

fn to_u32(string: &str) -> u32 {
    u32::from_str(&string).unwrap_or(0)
}

pub struct Claim {
    pub id: u32,
    pub rectangle: Rectangle,
}

impl Claim {
    pub fn new(string: String) -> Self {
        let re = Regex::new(r"^#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();

        let cap = re.captures(&string).unwrap();

        Claim {
            id: to_u32(&cap[1]),
            rectangle: Rectangle::new(
                to_u32(&cap[2]),
                to_u32(&cap[3]),
                to_u32(&cap[4]),
                to_u32(&cap[5]),
            ),
        }
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        let rect = &self.rectangle;

        x >= rect.top_left.x
            && x < rect.bottom_right.x
            && y >= rect.top_left.y
            && y < rect.bottom_right.y
    }

    pub fn overlaps(&self, other: &Claim) -> bool {
        let a = &self.rectangle;
        let b = &other.rectangle;

        a.top_left.x < b.bottom_right.x
            && a.bottom_right.x > b.top_left.x
            && a.top_left.y < b.bottom_right.y
            && a.bottom_right.y > b.top_left.y
    }

    pub fn width(&self) -> u32 {
        &self.rectangle.width + &self.rectangle.top_left.x
    }

    pub fn height(&self) -> u32 {
        &self.rectangle.height + &self.rectangle.top_left.y
    }
}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Rectangle {
            top_left: Point::new(x, y),
            bottom_right: Point::new(x + width, y + height),
            width,
            height,
        }
    }
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }
}
