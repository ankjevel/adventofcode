use std::io::Result;

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    Ok(input
        .to_owned()
        .into_iter()
        .map(|(time, distance)| {
            (0..=time)
                .into_iter()
                .filter(|warmup| {
                    let runtime = time - warmup;
                    let total_time = runtime * warmup;
                    total_time > distance
                })
                .collect::<Vec<_>>()
                .len()
        })
        .product())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = include_str!("../../input/day_06_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 288);
        Ok(())
    }
}
