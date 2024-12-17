pub mod part_01;
pub mod part_02;
pub mod square;

use std::collections::BTreeMap;

use square::{
    Guard,
    Square::{self, Floor, Obstruction},
};

pub type Pos = isize;
pub type Map = BTreeMap<(Pos, Pos), Square>;
pub type Input = (Guard, Map);

pub fn parse_input(input: &str) -> Input {
    let mut current_position = (0, 0);
    let map = input
        .lines()
        .map(str::trim)
        .filter(|string| !string.is_empty())
        .map(str::to_owned)
        .enumerate()
        .flat_map(|(y, string)| {
            string
                .chars()
                .enumerate()
                .map(|(x, square)| {
                    let x_y = (x as Pos, y as Pos);
                    (
                        x_y,
                        match square {
                            '#' => Obstruction,
                            '^' => {
                                current_position = x_y;
                                Floor
                            }
                            _ => Floor,
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (Guard::new(current_position), map)
}

#[cfg(test)]
mod tests {
    use square::Guard;

    use super::*;

    const EXAMPLE_DATA: &str = "
        .^
        #.
    ";

    #[test]
    fn it_parses_example() {
        let expected: Input = (
            Guard::new((1, 0)),
            vec![
                ((0, 0), Floor),
                ((1, 0), Floor),
                ((0, 1), Obstruction),
                ((1, 1), Floor),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(parse_input(EXAMPLE_DATA), expected);
    }
}
