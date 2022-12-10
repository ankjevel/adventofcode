use std::io::Result;

use crate::Input;

pub fn main(_input: &Input) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        let input = parse_input(include_str!("../../input/day_10_extended_example"));

        let _ = main(&input)?;

        Ok(())
    }
}
