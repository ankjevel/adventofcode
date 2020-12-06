use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    let mut entries = (0, 0);
    'outer: for a in input {
        for b in input.into_iter() {
            if a + b == 2020 {
                entries = (a.to_owned(), b.to_owned());
                break 'outer;
            }
        }
    }

    Ok(entries.0 * entries.1)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        1721
        979
        366
        299
        675
        1456
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 514579);
        Ok(())
    }
}
