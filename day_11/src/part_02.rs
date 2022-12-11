use std::io::Result;

use crate::{evaluate::evaluate, Input};

pub fn main(input: &Input, iterations: i32) -> Result<i64> {
    Ok(evaluate(input, iterations, 1))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        let input = parse_input(include_str!("../../input/day_11_extended_example"));
        assert_eq!(main(&input, 1)?, 24);
        assert_eq!(main(&input, 20)?, 10197);
        assert_eq!(main(&input, 1000)?, 27019168);
        assert_eq!(main(&input, 2000)?, 108263829);
        Ok(())
    }
}
