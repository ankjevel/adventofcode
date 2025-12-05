use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<u64> {
    Ok(input
        .iter()
        .map(|bank| {
            let mut max = 0;
            for i in 0..bank.len() {
                for j in (i + 1)..bank.len() {
                    let joltage = format!("{}{}", bank[i], bank[j]).parse().unwrap();
                    if joltage > max {
                        max = joltage;
                    }
                }
            }
            max
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_03_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 357);
        Ok(())
    }
}
