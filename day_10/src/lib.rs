#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate regex;

pub mod board;
pub mod point;

use point::Position;
use regex::{Captures, Regex};

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"position=<(?:(?:\s+)?(-?\d+)),(?:(?:\s+)?(-?\d+))> velocity=<(?:(?:\s+)?(-?\d+)),(?:(?:\s+)?(-?\d+))>"
    )
    .unwrap();
}

pub type Location = (isize, isize);

fn min_max(positions: &Vec<Position>) -> (Location, Location) {
    positions
        .iter()
        .fold(((0, 0), (0, 0)), |mut min_max, point| {
            let (x, y) = (point.x as isize, point.y as isize);
            if x < min_max.0 .0 {
                min_max.0 .0 = x;
            }
            if y < min_max.0 .1 {
                min_max.0 .1 = y;
            }
            if x > min_max.1 .0 {
                min_max.1 .0 = x;
            }
            if y > min_max.1 .1 {
                min_max.1 .1 = y;
            }
            min_max
        })
}

type Input = (i128, i128, i128, i128);

pub fn parse_input(string: &str) -> Vec<Input> {
    fn to_number(input: &Captures, index: usize) -> i128 {
        if let Some(string) = input.get(index) {
            string.as_str().parse::<_>().unwrap()
        } else {
            0
        }
    }

    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| {
            if !RE.is_match(&string) {
                panic!("none matching for {}", string);
            }
            let cap = RE.captures(&string).unwrap();

            (
                to_number(&cap, 1),
                to_number(&cap, 2),
                to_number(&cap, 3),
                to_number(&cap, 4),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("test_fixture.txt");

    #[test]
    fn it_parses_the_example_data() {
        parse_input(&EXAMPLE_DATA);
    }
}
