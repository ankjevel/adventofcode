use std::str::FromStr;
use regex::Regex;

pub struct Claim {
    pub id: i32,
    pub rectangle: Rectangle,
}

impl Claim {
    fn to_i32 (string: &str) -> i32 {
        i32::from_str(&string).unwrap_or(0)
    }

    pub fn new(string: String) -> Self {
        let re = Regex::new(r"^#(\d*) @ (\d*),(\d*): (\d*)x(\d*)$").unwrap();

        let cap = re.captures(&string).unwrap();

        Claim {
            id: Claim::to_i32(&cap[1]),
            rectangle: Rectangle::new(
                Claim::to_i32(&cap[2]),
                Claim::to_i32(&cap[3]),
                Claim::to_i32(&cap[4]),
                Claim::to_i32(&cap[5]),
            )
        }
    }
}

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32
    ) -> Self {
        Rectangle {
            top_left: Point::new(
                x,
                y
            ),
            bottom_right: Point::new(
                x + width,
                y + height
            ),
            width,
            height,
        }
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
