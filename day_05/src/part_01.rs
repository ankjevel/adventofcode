use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    Ok(input
        .ingredients
        .iter()
        .filter(|ingredient| {
            input
                .ranges
                .iter()
                .find(|range| range.contains(&ingredient))
                .is_some()
        })
        .collect::<Vec<_>>()
        .len())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_05_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 3);
        Ok(())
    }
}
