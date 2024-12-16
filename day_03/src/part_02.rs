use std::io::Result;

use crate::{Input, Instruction, Number};

pub fn main(input: &Input) -> Result<Number> {
    let mut should_skip = false;
    Ok(input
        .iter()
        .filter(|row| {
            if should_skip {
                if row.skip.is_none() || row.skip.unwrap() {
                    should_skip = true;
                    return false;
                }
                should_skip = false;
            } else if row.skip.is_some() {
                should_skip = row.skip.unwrap();
                return false;
            }

            row.mul.is_some()
        })
        .map(Instruction::sum)
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 48);
        Ok(())
    }
}
