use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    let mut input = input.clone().to_owned();
    input.sort();
    input.push(*input.iter().max().unwrap() + 3);

    let mut iter = input.iter_mut().peekable();
    let mut jolt = 0;
    let mut diff = (0, 0);
    while iter.len() > 0 {
        if let Some(next) = iter.peek() {
            if !(jolt + 1..=jolt + 3).contains(*next) {
                break;
            }
        }
        let adapter = iter.next().unwrap();
        match *adapter - jolt {
            1 => diff.0 += 1,
            3 => diff.1 += 1,
            _ => (),
        }
        jolt = *adapter;
    }
    let (one_jolt, three_jolt) = diff;
    Ok(one_jolt * three_jolt)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 35);
        Ok(())
    }
}
