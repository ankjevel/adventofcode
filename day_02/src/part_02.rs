use std::io::Result;

use crate::Hand::*;
use crate::Input;

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .into_iter()
        .map(|(opponent, outcome)| match outcome {
            // lose
            Rock => {
                0 + match opponent {
                    Rock => 3,
                    Paper => 1,
                    Scissors => 2,
                }
            }
            // draw
            Paper => {
                3 + match opponent {
                    Rock => 1,
                    Paper => 2,
                    Scissors => 3,
                }
            }
            // win
            Scissors => {
                6 + match opponent {
                    Rock => 2,
                    Paper => 3,
                    Scissors => 1,
                }
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        A Y
        B X
        C Z
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 12);
        Ok(())
    }
}
