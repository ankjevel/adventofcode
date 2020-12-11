use std::collections::BTreeMap;

pub mod part_01;
pub mod part_02;
pub mod tile;
pub mod x_y;

use tile::Tile;

pub type Input = BTreeMap<(usize, usize), Tile>;

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .enumerate()
        .map(|(y, string)| {
            string
                .chars()
                .enumerate()
                .map(|(x, position)| ((x, y), Tile::new(&position)))
                .collect::<Vec<((usize, usize), Tile)>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use tile::Tile::*;

    const EXAMPLE_DATA: &'static str = "
        L.L
        #.L
    ";

    #[test]
    fn it_parses_example() {
        let expected: Input = vec![
            ((0, 0), Empty),
            ((1, 0), Floor),
            ((2, 0), Empty),
            ((0, 1), Occupied),
            ((1, 1), Floor),
            ((2, 1), Empty),
        ]
        .into_iter()
        .collect();
        assert_eq!(parse_input(&EXAMPLE_DATA), expected);
    }
}
