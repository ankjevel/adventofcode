use std::io::Result;

use crate::{Direction::*, Input};

pub fn main(input: &Input) -> Result<u16> {
    let nodes = input.nodes.to_owned();
    let index_of_aaa = *input.indices.get("AAA").unwrap();
    let mut current = nodes.borrow().get(index_of_aaa).unwrap().to_owned();

    let mut steps = 0;
    'main: loop {
        for direction in input.moves.clone() {
            steps += 1;

            let next_index = match direction {
                Left => &*current.1 .0,
                Right => &*current.1 .1,
            };

            let next_index = *input.indices.get(next_index).unwrap();
            let next = nodes.borrow().get(next_index).unwrap().to_owned();

            if next.0 == "ZZZ" {
                break 'main;
            }

            current = next;
        }
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    ";

    const LONGER_EXAMPLE_DATA: &'static str = include_str!("../../input/day_08_example");

    #[test]
    fn it_gets_the_short_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 6);
        Ok(())
    }

    #[test]
    fn it_gets_the_longer_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&LONGER_EXAMPLE_DATA))?, 2);
        Ok(())
    }
}
