use std::io::Result;

use crate::{math::lcm, Input, Integer};

pub fn main(input: &Input) -> Result<Integer> {
    Ok(input
        .indices
        .keys()
        .into_iter()
        .filter(|k| k.ends_with('A'))
        .map(|k| *input.indices.get(k).unwrap())
        .map(|start_index| Input::solve(input, start_index))
        .fold(1, |acc, n| lcm(acc, n)))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 6);
        Ok(())
    }
}
