use std::{collections::HashMap, io::Result};

use crate::Input;

fn naive_recursion(
    jolt: u64,
    index: usize,
    adapters: &Vec<u64>,
    memo: &mut HashMap<u64, u64>,
) -> u64 {
    let len = adapters.len();
    if len == index {
        return 1;
    }

    if let Some(c) = memo.get(&jolt) {
        return *c;
    }

    let combinations = (index..=index + 3)
        .filter(|i| i < &len)
        .map(|i| (i, *adapters.get(i).unwrap()))
        .filter(|(_, adapter)| *adapter <= jolt + 3)
        .map(|(i, adapter)| naive_recursion(adapter, i + 1, adapters, memo))
        .sum();
    memo.insert(jolt, combinations);
    combinations
}

pub fn main(input: &Input) -> Result<u64> {
    let mut adapters: Vec<_> = input.iter().map(|adapter| *adapter as u64).collect();

    adapters.sort();
    adapters.push(*adapters.last().unwrap() + 3);

    let mut memo = HashMap::new();
    Ok(naive_recursion(0, 0, &adapters, &mut memo))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const SHORT_EXAMPLE_DATA: &'static str = "
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
    fn it_gets_the_first_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&SHORT_EXAMPLE_DATA))?, 8);
        Ok(())
    }

    const LONGER_EXAMPLE_DATA: &'static str = "
        28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3
    ";

    #[test]
    fn it_gets_the_longer_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&LONGER_EXAMPLE_DATA))?, 19208);
        Ok(())
    }
}
