use std::io::Result;

use crate::{evaluate::evaluate, Input};

pub fn main(input: &Input) -> Result<i64> {
    Ok(evaluate(input, 20, 3))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        let input = parse_input(include_str!("../../input/day_11_extended_example"));
        assert_eq!(main(&input)?, 10605);
        Ok(())
    }
}
