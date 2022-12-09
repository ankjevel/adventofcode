pub mod direction;
pub mod knot;
pub mod part_01;
pub mod part_02;
pub mod print;

use direction::{Direction, Direction::*};

pub type Input = Vec<(Direction, usize)>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .map(|row| {
            let n: Vec<_> = row.split(" ").into_iter().map(str::to_owned).collect::<_>();
            let direction = match &*n[0] {
                "L" => Left,
                "R" => Right,
                "U" => Up,
                _ => Down,
            };
            let steps = n[1].parse::<_>().unwrap_or(0);
            (direction, steps)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &'static str = "
        L 10
        R 2
        U 14
        D 1030
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![(Left, 10), (Right, 2), (Up, 14), (Down, 1030)]
        );
    }
}
