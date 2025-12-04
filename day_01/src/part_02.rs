use std::io::Result;

use crate::{Input, MAX, Number};

pub fn main(input: &Input) -> Result<Number> {
    let mut zeros = 0;
    let mut dial = 50;

    for rotation in input {
        let turn = if rotation > &0 { -1 } else { 1 };
        for _ in 0..rotation.abs() {
            let mut new_value = (dial + turn) % MAX;
            if new_value > MAX {
                new_value = 0;
            }

            if new_value == 0 {
                zeros += 1;
            }

            dial = new_value;
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
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 6);
        Ok(())
    }
}
