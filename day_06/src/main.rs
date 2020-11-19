extern crate regex;

mod part_01;
mod part_02;
mod point;

use point::Point;
use std::io;

fn main() -> io::Result<()> {
    let coords = parse_input(include_str!("../../input/day_06"));

    println!("part_01: {}", part_01::main(&coords).unwrap());
    println!("part_02: {}", part_02::main(&coords, 10000).unwrap());

    Ok(())
}

fn parse_input(string: &str) -> Vec<Point> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| Point::new(&string))
        .collect::<Vec<Point>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
    1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9";

    #[test]
    fn what_is_the_size_of_the_largest_area() {
        let result = part_01::main(&parse_input(&EXAMPLE_DATA)).unwrap();

        assert_eq!(result, 17)
    }

    #[test]
    fn total_distance() {
        let result = part_02::main(&parse_input(&EXAMPLE_DATA), 32).unwrap();

        assert_eq!(result, 16)
    }
}
