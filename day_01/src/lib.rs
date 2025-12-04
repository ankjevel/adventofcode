pub mod part_01;
pub mod part_02;

pub type Number = i32;
pub type Input = Vec<Number>;

pub const MAX: Number = 100;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(|part| {
            let (direction, clicks_str) = part.split_at(1);
            let click: Number = clicks_str.parse().unwrap();
            if direction == "L" { -click } else { click }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        L5
        L55
        R5
        R55
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(parse_input(EXAMPLE_DATA), vec![-5, -55, 5, 55]);
    }
}
