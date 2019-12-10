#[macro_use]
extern crate float_cmp;

pub mod angle;
pub mod part_01;
pub mod part_02;
pub mod point;

pub type Input = Vec<Vec<char>>;

pub fn to_points(input: &Input) -> Vec<point::Point> {
    let mut points = Vec::new();
    for (y, row) in input.into_iter().enumerate() {
        for (x, item) in row.into_iter().enumerate() {
            if item == &'.' {
                continue;
            }

            points.push(point::Point::from_usize(x, y));
        }
    }

    points
}

pub fn parse_input(string: &str) -> Input {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| string.chars().collect())
        .collect()
}
