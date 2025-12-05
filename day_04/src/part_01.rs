use std::io::Result;

use crate::{Input, Position, can_move};

pub fn main(input: &Input) -> Result<usize> {
    Ok(input
        .iter()
        .enumerate()
        .map(|(row, columns)| {
            columns
                .iter()
                .enumerate()
                .filter(|(column, current)| {
                    let valid = **current == Position::PaperRoll && can_move(input, row, *column);

                    if valid {
                        print!("x");
                    } else if **current == Position::PaperRoll {
                        print!("@")
                    } else {
                        print!(".")
                    }

                    if *column == columns.len() - 1 {
                        print!("\n")
                    }

                    valid
                })
                .collect::<Vec<_>>()
                .len()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_04_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 13);
        Ok(())
    }
}
