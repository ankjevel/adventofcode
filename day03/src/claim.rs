use std::str::FromStr;
use regex::Regex;

fn to_i32 (string: &str) -> i32 {
    i32::from_str(&string).unwrap_or(0)
}

pub struct Claim {
    pub id: i32,
    pub rectangle: Rectangle
}

impl Claim {
    pub fn new(string: String) -> Self {
        let re = Regex::new(r"^#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();

        let cap = re.captures(&string).unwrap();

        Claim {
            id: to_i32(&cap[1]),
            rectangle: Rectangle::new(
                to_i32(&cap[2]),
                to_i32(&cap[3]),
                to_i32(&cap[4]),
                to_i32(&cap[5])
            )
        }
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        let rect = &self.rectangle;

        x >= rect.top_left.x &&
        x <= (rect.bottom_right.x - 1) &&
        y >= rect.top_left.y &&
        y <= (rect.bottom_right.y - 1)
    }

    pub fn overlaps(&self, other: &Claim) -> bool {
        let a = &self.rectangle;
        let b = &other.rectangle;

        a.top_left.x <= (b.bottom_right.x - 1) &&
        (a.bottom_right.x - 1) >= b.top_left.x &&
        a.top_left.y <= (b.bottom_right.y - 1) &&
        (a.bottom_right.y - 1) >= b.top_left.y
    }


    pub fn width(&self) -> i32 {
        &self.rectangle.width + &self.rectangle.top_left.x
    }

    pub fn height(&self) -> i32 {
       &self.rectangle.height + &self.rectangle.top_left.y
    }

}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
    pub width: i32,
    pub height: i32
}

impl Rectangle {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rectangle {
            top_left: Point::new(x, y),
            bottom_right: Point::new(x + width, y + height),
            width,
            height
        }
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
