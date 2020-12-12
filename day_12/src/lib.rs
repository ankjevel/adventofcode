pub mod action;
pub mod direction;
pub mod facing;
pub mod manhattan;
pub mod point;
pub mod ship;

pub mod part_01;
pub mod part_02;

use action::Action;

pub type Input = Vec<(Action, u16)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|string| string.chars().collect::<Vec<_>>())
        .map(|chars| {
            let action = Action::new(chars[0]);
            let values: String = chars[1..].into_iter().collect();
            (action, values.parse::<_>().unwrap_or(0))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Action::*;

    const EXAMPLE_DATA: &'static str = "
        F10
        N3
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(&EXAMPLE_DATA), vec![(Forward, 10), (North, 3)]);
    }
}
