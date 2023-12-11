use std::io::Result;

use crate::{Input, Integer};

pub fn main(input: &Input) -> Result<Integer> {
    Ok(Input::solve(input, *input.indices.get("AAA").unwrap()))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    ";

    const LONGER_EXAMPLE_DATA: &'static str = include_str!("../../input/day_08_example");

    #[test]
    fn it_gets_the_short_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 6);
        Ok(())
    }

    #[test]
    fn it_gets_the_longer_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&LONGER_EXAMPLE_DATA))?, 2);
        Ok(())
    }
}
