use std::{cmp::Ordering, io::Result};

use crate::{Input, Number};

const MAX: usize = 12;

pub fn main(input: &Input) -> Result<Number> {
    Ok(input
        .iter()
        .map(|bank| {
            let mut index_skip = 0;
            (1..=MAX)
                .map(|index| {
                    let (index_of_label, label) = bank
                        [(index - 1 + index_skip)..bank.len() - (MAX - index)]
                        .iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| if a == b { Ordering::Greater } else { a.cmp(b) })
                        .unwrap();
                    index_skip += index_of_label;
                    label.to_string()
                })
                .collect::<Vec<_>>()
                .join("")
                .parse::<Number>()
                .unwrap()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 3121910778619);
        Ok(())
    }
}
