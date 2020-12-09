use std::io::Result;

use crate::Input;

pub fn get_invalid(input: &Input, preamble_length: usize) -> u64 {
    let preamble_length = preamble_length + 1;
    let len = input.len();
    let mut skip_index = 0;

    'check: while skip_index + preamble_length < len {
        let mut preamble = input
            .into_iter()
            .skip(skip_index)
            .take(preamble_length)
            .map(u64::to_owned)
            .collect::<Vec<_>>();
        let last = preamble.pop().unwrap();
        for a in &preamble {
            for b in &preamble {
                if a == b {
                    continue;
                }
                if a + b == last {
                    skip_index += 1;
                    continue 'check;
                }
            }
        }
        return last.to_owned();
    }

    0
}

pub fn main(input: &Input, preamble_length: usize) -> Result<u64> {
    Ok(get_invalid(input, preamble_length))
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
        assert_eq!(main(&parse_input(&EXAMPLE_DATA), 5)?, 127);
        Ok(())
    }
}
