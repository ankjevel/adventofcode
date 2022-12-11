use std::io::Result;

use crate::Input;

pub fn main(_input: &Input) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        example
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, ());
        Ok(())
    }
}
