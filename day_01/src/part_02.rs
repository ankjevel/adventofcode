use std::io::Result;

use crate::Input;

pub fn main(_input: &Input) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 5);
        Ok(())
    }
}
