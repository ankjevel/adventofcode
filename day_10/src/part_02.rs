use std::io::Result;

use crate::Input;

fn naive_recursion(jolt: u32, index: usize, adapters: &Vec<u32>) -> u64 {
    let len = adapters.len();
    if len == index {
        return 1;
    }
    (index..=index + 3)
        .filter(|i| i < &len && *adapters.get(*i).unwrap() <= jolt + 3)
        .map(|i| naive_recursion(*adapters.get(i).unwrap(), i + 1, adapters))
        .sum()
}

pub fn main(input: &Input) -> Result<u64> {
    let mut adapters = input.clone().to_owned();
    adapters.sort();
    adapters.push(*adapters.iter().max().unwrap() + 3);

    Ok(naive_recursion(0, 0, &adapters))
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
