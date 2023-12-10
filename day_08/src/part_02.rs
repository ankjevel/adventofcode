use std::{cell::RefCell, io::Result};

use crate::{Direction::*, Input};

pub fn main(input: &Input) -> Result<u64> {
    let nodes = input.nodes.to_owned();
    let all_indices = input
        .indices
        .keys()
        .into_iter()
        .filter(|k| k.ends_with('A'))
        .map(|k| *input.indices.get(k).unwrap())
        .collect::<Vec<_>>();

    let ends_to_find = all_indices.len();

    let all_currents = RefCell::new(
        all_indices
            .clone()
            .into_iter()
            .map(|i| nodes.borrow().get(i).unwrap().to_owned())
            .collect::<Vec<_>>(),
    );

    let mut steps = 0;
    'main: loop {
        for direction in input.moves.clone() {
            steps += 1;

            let found_ends =
                all_currents
                    .borrow_mut()
                    .iter_mut()
                    .fold(0, |mut completed, current| {
                        let next_index = match direction {
                            Left => &*current.1 .0,
                            Right => &*current.1 .1,
                        };
                        let next_index = *input.indices.get(next_index).unwrap();
                        let next = nodes.borrow().get(next_index).unwrap().to_owned();

                        *current = next;

                        if current.0.ends_with('Z') {
                            completed += 1;
                        }

                        completed
                    });

            if found_ends == ends_to_find {
                break 'main;
            }
        }
    }

    Ok(steps)
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
