use std::io::Result;

use crate::{Input, MAX};

pub fn main(input: &Input) -> Result<usize> {
    let mut zeros = 0;
    let mut dial = 50;

    for rotation in input {
        let mut new = dial + rotation;
        if new < 0 {
            new = MAX - new.abs();
        }
        dial = new % MAX;
        if dial == 0 {
            zeros += 1;
        }
    }

    Ok(zeros)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 3);
        Ok(())
    }
}
