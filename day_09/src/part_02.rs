use std::io::Result;

use crate::{part_01::get_invalid, Input};

pub fn min_max(input: &Input, invalid: u64) -> u64 {
    let input = input.clone().iter().map(u64::to_owned).collect::<Vec<_>>();
    let len = input.len();
    let mut index = 0;
    while index < len {
        let n = &input
            .iter()
            .skip(index)
            .map(u64::to_owned)
            .filter(|n| n != &invalid)
            .collect::<Vec<_>>();
        let nn = n.len();
        let mut i = 0;
        while i < nn {
            let sample = n.into_iter().take(i).map(u64::to_owned).collect::<Vec<_>>();
            let sum = sample.iter().sum::<u64>();

            if sum != invalid {
                i += 1;
                continue;
            }

            let min = sample.iter().min().unwrap().to_owned();
            let max = sample.iter().max().unwrap().to_owned();

            return min + max;
        }
        index += 1;
    }
    0
}

pub fn main(input: &Input, preamble_length: usize) -> Result<u64> {
    Ok(min_max(input, get_invalid(input, preamble_length)))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA), 5)?, 62);
        Ok(())
    }
}
