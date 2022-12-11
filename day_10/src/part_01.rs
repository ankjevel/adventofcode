use std::io::Result;

use crate::{cpu::CPU, Input};

pub fn main(input: &Input) -> Result<i64> {
    let mut cpu = CPU::new();
    cpu.parse(input);

    Ok(vec![20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|index| (index as i64) * cpu.stack.get(&index).unwrap_or(&1).to_owned())
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        let input = parse_input(include_str!("../../input/day_10_extended_example"));
        assert_eq!(main(&input)?, 13140);

        Ok(())
    }
}
