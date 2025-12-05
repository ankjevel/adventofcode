use std::io::Result;

use crate::{Input, Number};

fn repeated(input: &Number) -> bool {
    let string = input.to_string();
    for i in 1..=(string.len() / 2) {
        let arr = string
            .chars()
            .collect::<Vec<_>>()
            .chunks(i)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        let first = arr.first().unwrap();
        if arr.iter().all(|item| item == first) {
            return true;
        }
    }
    false
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

    const EXAMPLE_DATA: &str = include_str!("../../input/day_02_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 4174379265);
        Ok(())
    }
}
