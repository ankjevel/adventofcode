use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(string: &str) -> Self {
        let re = Regex::new(r"(\d*), (\d*)").unwrap();
        let cap = re.captures(&string).unwrap();

        let x = cap.get(1).map_or(0, |s| i32::from_str(s.as_str()).unwrap());
        let y = cap.get(2).map_or(0, |s| i32::from_str(s.as_str()).unwrap());

        Point { x, y }
    }
}
