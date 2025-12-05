use std::io::Result;

use crate::{Input, Position, can_move};

pub fn main(input: &Input) -> Result<usize> {
    let mut movements = 0;
    let mut copy = input.clone();
    'main: loop {
        let mut modified = false;
        let mut modified_copy = copy.to_owned();

        copy.iter().enumerate().for_each(|(row, columns)| {
            let removed = columns
                .iter()
                .enumerate()
                .filter_map(|(column, current)| {
                    let valid = *current == Position::PaperRoll && can_move(&copy, row, column);

                    if valid {
                        print!("\x1b[31m@\x1b[0m");
                    } else if *current == Position::PaperRoll {
                        print!("@")
                    } else {
                        print!(".")
                    }

                    if column == columns.len() - 1 {
                        print!("\n")
                    }

                    if valid { Some(column) } else { None }
                })
                .collect::<Vec<_>>();

            if removed.len() != 0 {
                movements += removed.len();
                *modified_copy.get_mut(row).unwrap() = columns
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if removed.iter().find(|j| i == **j).is_none() {
                            v.to_owned()
                        } else {
                            Position::Empty
                        }
                    })
                    .collect::<Vec<_>>();
                modified = true
            }
        });

        if modified == false {
            break 'main;
        } else {
            copy = modified_copy;
            println!("");
        }
    }

    Ok(movements)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_04_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 43);
        Ok(())
    }
}
