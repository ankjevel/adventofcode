use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    let mut last = None;
    let mut measurement_increases = 0;
    for value in input {
        match last {
            Some(pre_value) => {
                if &pre_value < value {
                    measurement_increases += 1;
                }
            }
            _ => {}
        }
        last = Some(value.to_owned());
    }
    Ok(measurement_increases)
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
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 7);
        Ok(())
    }
}
