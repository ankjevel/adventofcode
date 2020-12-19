use std::io::Result;

use crate::{memory_game::play, Input};

pub fn main(input: &Input) -> Result<u64> {
    Ok(play(input, 2020))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input("0,3,6"))?, 436);
        Ok(())
    }
}
