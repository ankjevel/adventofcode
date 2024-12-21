use std::{io::Result, mem};

use crate::{Input, Number};

pub fn main(input: &Input) -> Result<Number> {
    Ok(input
        .iter()
        .filter_map(|(test_value, operators)| {
            let mut operators = operators.iter().to_owned();
            let mut calibrations = vec![operators.next().unwrap().to_owned()];
            for operator in operators {
                let new_calibrations = calibrations
                    .iter()
                    .flat_map(|calibration| {
                        vec![
                            calibration + operator,
                            calibration * operator,
                            format!("{calibration}{operator}").parse().unwrap(),
                        ]
                    })
                    .collect();
                let _ = mem::replace(&mut calibrations, new_calibrations);
            }

            let calibrations = calibrations
                .iter()
                .filter(|result| test_value == *result)
                .collect::<Vec<_>>();

            if calibrations.is_empty() {
                None
            } else {
                Some(test_value)
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_07_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 11387);
        Ok(())
    }
}
