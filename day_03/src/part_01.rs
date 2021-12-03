use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    let bits = (0..input.first().unwrap().len())
        .into_iter()
        .map(|n| {
            input
                .into_iter()
                .map(|v| v.chars().collect::<Vec<_>>()[n].to_owned())
                .collect::<Vec<_>>()
        })
        .map(|v| {
            let mut result = vec![(0u8, 0u32), (1u8, 0u32)];
            v.iter().for_each(|n| match n {
                '1' => result[1].1 += 1,
                _ => result[0].1 += 1,
            });
            result
        })
        .collect::<Vec<Vec<_>>>();

    let gamma = bits
        .clone()
        .iter()
        .map(
            |vec| match vec.iter().max_by_key(|n| n.1.to_owned()).unwrap().0 {
                1 => '1',
                _ => '0',
            },
        )
        .collect::<String>();

    let epsilon = bits
        .clone()
        .iter()
        .map(
            |vec| match vec.iter().min_by_key(|n| n.1.to_owned()).unwrap().0 {
                1 => '1',
                _ => '0',
            },
        )
        .collect::<String>();

    Ok(u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 198);
        Ok(())
    }
}
