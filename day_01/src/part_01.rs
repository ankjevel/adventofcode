use std::io::Result;

use crate::{Input, Number};

pub fn main(input: &Input) -> Result<Number> {
    let (mut left, mut right) = input.clone();

    left.sort();
    right.sort();

    Ok(left.iter().enumerate().fold(0, |sum, (index, left_value)| {
        sum + (left_value - right.get(index).unwrap()).abs()
    }))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 11);
        Ok(())
    }
}
