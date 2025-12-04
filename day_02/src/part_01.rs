use std::io::Result;

use crate::{Input, Number};

fn repeated(input: &Number) -> bool {
    let string = input.to_string();
    let (left, right) = string.split_at(string.len() / 2);
    left == right
}

pub fn main(input: &Input) -> Result<Number> {
    let mut sum = 0;

    for (left, right) in input.to_owned() {
        for val in left..=right {
            if repeated(&val) {
                sum += val;
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 1227775554);
        Ok(())
    }
}
