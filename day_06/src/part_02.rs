use std::{collections::HashSet, io::Result, thread};

use crate::{
    square::{Guard, Square::*},
    Input,
};

pub fn main((guard, map): &Input) -> Result<usize> {
    let start_position = guard.position;
    Ok(map
        .iter()
        .filter_map(|(pos, square)| {
            if pos != &start_position && square == &Floor {
                Some(pos.to_owned())
            } else {
                None
            }
        })
        .map(|pos| {
            let mut map = map.to_owned();
            let mut guard = Guard::new(start_position);
            let mut visited = HashSet::from([guard.current()]);

            map.insert(pos, Obstruction);

            thread::spawn(move || loop {
                let next = guard.next();
                if let Some(square) = map.get(&next) {
                    let key = (next, guard.facing);
                    if visited.contains(&key) {
                        return true;
                    } else if square == &Obstruction {
                        guard.turn();
                    } else {
                        guard.set_position(next);
                        visited.insert(key);
                    }
                } else {
                    return false;
                }
            })
        })
        .map(|handle| handle.join().unwrap())
        .filter(|loop_encountered| *loop_encountered)
        .collect::<Vec<_>>()
        .len())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_06_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 6);
        Ok(())
    }
}
