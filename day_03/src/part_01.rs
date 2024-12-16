use std::io::Result;

use crate::{Input, Instruction, Number};

pub fn main(input: &Input) -> Result<Number> {
    Ok(input.iter().map(Instruction::sum).sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 161);
        Ok(())
    }
}
