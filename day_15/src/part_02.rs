use std::io::Result;

use crate::{memory_game::play, Input};

pub fn main(input: &Input) -> Result<u64> {
    Ok(play(input, 30000000))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_first_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input("0,3,6"))?, 175594);
        Ok(())
    }

    #[test]
    fn it_gets_the_rest_correct() -> Result<()> {
        assert_eq!(main(&parse_input("1,3,2"))?, 2578);
        assert_eq!(main(&parse_input("2,1,3"))?, 3544142);
        assert_eq!(main(&parse_input("1,2,3"))?, 261214);
        assert_eq!(main(&parse_input("2,3,1"))?, 6895259);
        assert_eq!(main(&parse_input("3,2,1"))?, 18);
        assert_eq!(main(&parse_input("3,1,2"))?, 362);
        Ok(())
    }
}
