use std::io::Result;

use crate::{hand::sort_hand, sum, Input};

pub fn main(input: &Input) -> Result<u128> {
    Ok(sum(input, Box::new(sort_hand)))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("../../input/day_07_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 6440);
        Ok(())
    }
}
