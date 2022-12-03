use std::collections::HashSet;
use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .into_iter()
        .map(|val| {
            let (left, right) = val.split_at(val.len() / 2);
            let left_chars = left.chars().into_iter();
            let right_chars = right.chars().into_iter();
            left_chars
                .into_iter()
                .filter_map(|char| {
                    if let Some(result) = right_chars.to_owned().find(|r_char| r_char == &char) {
                        Some(result as u32)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<u32>>()
                .iter()
                .map(|n| if n > &90 { n - 96 } else { n - 38 })
                .sum::<u32>()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 157);
        Ok(())
    }
}
