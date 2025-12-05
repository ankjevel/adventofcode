pub mod part_01;
pub mod part_02;

pub type Input = Vec<Vec<Position>>;

pub fn can_move(input: &Input, row: usize, column: usize) -> bool {
    let movements: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let empty = vec![];
    movements
        .iter()
        .filter(|movement| {
            input
                .get(((row as isize) + movement.0) as usize)
                .unwrap_or(&empty)
                .get(((column as isize) + movement.1) as usize)
                .unwrap_or(&Position::Empty)
                .eq(&Position::PaperRoll)
        })
        .collect::<Vec<_>>()
        .len()
        < 4
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Position {
    Empty,
    PaperRoll,
}

use Position::{Empty, PaperRoll};

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .into_iter()
        .map(|string| {
            string
                .chars()
                .into_iter()
                .map(|position| if position == '@' { PaperRoll } else { Empty })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "
        .@
        @.
    ";

    #[test]
    fn it_parses_example() {
        assert_eq!(
            parse_input(EXAMPLE_DATA),
            vec![vec![Empty, PaperRoll], vec![PaperRoll, Empty],]
        );
    }
}
