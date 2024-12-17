use std::{collections::HashSet, io::Result};

use crate::{square::Square::Obstruction, Input};

pub fn main((guard, map): &Input) -> Result<usize> {
    let mut guard = guard.to_owned();
    let mut visited = HashSet::from([guard.position]);

    loop {
        let next = guard.next();
        if let Some(square) = map.get(&next) {
            if square == &Obstruction {
                guard.turn();
            } else {
                guard.set_position(next);
                visited.insert(next);
            }
        } else {
            break;
        }
    }

    Ok(visited.len())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_06_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 41);
        Ok(())
    }
}
