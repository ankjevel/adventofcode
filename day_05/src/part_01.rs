use std::io::Result;

use crate::{Input, Integer};

pub fn main(input: &Input) -> Result<Integer> {
    Ok(input
        .seeds
        .clone()
        .into_iter()
        .map(|seed| Input::get_location(&input, seed))
        .min()
        .unwrap())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("../../input/day_05_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 35);
        Ok(())
    }
}
