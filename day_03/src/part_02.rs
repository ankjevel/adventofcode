use std::collections::HashSet;
use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .to_owned()
        .chunks(3)
        .map(|rows| {
            let chars: Vec<HashSet<char>> =
                rows.iter().map(|chars| chars.chars().collect()).collect();

            let (n0, n1, n2) = (
                chars[0].to_owned(),
                chars[1].to_owned(),
                chars[2].to_owned(),
            );

            n0.into_iter()
                .filter_map(|char| {
                    if n1.contains(&char) && n2.contains(&char) {
                        Some(char as u32)
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
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 70);
        Ok(())
    }
}
