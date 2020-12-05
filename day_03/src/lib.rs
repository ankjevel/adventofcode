pub mod part_01;
pub mod part_02;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Square {
    Open,
    Tree,
}

use Square::{Open, Tree};

pub fn parse_input(input: &str) -> Vec<Vec<Square>> {
    input
        .lines()
        .map(|string| string.trim())
        .filter(|string| !string.is_empty())
        .into_iter()
        .map(|string| {
            string
                .chars()
                .into_iter()
                .map(|square| if square == '.' { Open } else { Tree })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_input,
        Square::{Open, Tree},
    };

    const EXAMPLE_DATA: &'static str = "
        ..##.......
        #...#...#..
    ";

    #[test]
    fn it_can_parse_input() {
        assert_eq!(
            parse_input(&EXAMPLE_DATA),
            vec![
                vec![Open, Open, Tree, Tree, Open, Open, Open, Open, Open, Open, Open],
                vec![Tree, Open, Open, Open, Tree, Open, Open, Open, Tree, Open, Open]
            ]
        )
    }
}
