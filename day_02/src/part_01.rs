use std::io::Result;

use crate::{Input, Number};

fn repeated(input: &Number) -> bool {
    let string = input.to_string();
    let (left, right) = string.split_at(string.len() / 2);
    left == right
}

pub fn main(input: &Input) -> Result<Number> {
    let mut sum = 0;

    for (left, right) in input.to_owned() {
        for val in left..=right {
            if repeated(&val) {
                sum += val;
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_02_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 1227775554);
        Ok(())
    }
}
