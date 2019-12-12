extern crate day_09;

pub mod direction;
pub mod painting_robot;
pub mod point;

pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .next()
        .unwrap()
        .split(',')
        .map(|part| part.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
