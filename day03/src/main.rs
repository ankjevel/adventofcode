#![feature(label_break_value)]

use std::io::Result;

pub mod manhattan;
mod part_01;
pub mod structs;

use part_01::main as part_01;
use structs::Direction::{Down, Left, Right, Up};
use structs::Movement;

pub use structs::Point;

fn main() -> Result<()> {
    let wires = parse_input(include_str!("../../input/day_03"));

    println!("part_01: {:?}", part_01(&wires).unwrap());
    Ok(())
}

fn parse_input(string: &str) -> Vec<Vec<Point>> {
    string
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .map(|string| {
            let movement = string
                .split(',')
                .map(|part| Movement::new(part))
                .collect::<Vec<Movement>>();

            let mut position = Point::new();
            let mut points = Vec::new();

            for current in movement.iter() {
                match current.direction {
                    Down => position.y -= current.steps,
                    Left => position.x -= current.steps,
                    Right => position.x += current.steps,
                    Up => position.y += current.steps,
                }
                points.push(position.clone());
            }

            points
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_01: &'static str = "
        R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83
    ";

    const EXAMPLE_DATA_02: &'static str = "
        R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R77
    ";

    #[test]
    fn it_gets_the_correct_result_based_on_examples() {
        assert_eq!(159, part_01(&parse_input(EXAMPLE_DATA_01)).unwrap());
        assert_eq!(135, part_01(&parse_input(EXAMPLE_DATA_02)).unwrap());
    }
}
