use std::io::Result;

use crate::{is_safe, Input};

pub fn main(input: &Input) -> Result<usize> {
    Ok(input
        .iter()
        .filter(|row| {
            for i in 0..row.len() {
                let mut copy = row.to_owned().to_owned();
                copy.remove(i);
                if is_safe(&&copy) {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<_>>()
        .len())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 4);
        assert_eq!(main(&parse_input("86 87 90 92 91 94"))?, 1);
        Ok(())
    }
}
