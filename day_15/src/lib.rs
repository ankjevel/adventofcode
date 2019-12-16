#[macro_use]
extern crate float_cmp;

extern crate day_09;
extern crate rand;
extern crate termion;

pub type Point = (i64, i64);

pub mod enums;
pub mod paths;
pub mod repair_droid;

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
