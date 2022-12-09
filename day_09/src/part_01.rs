use std::io::Result;

use crate::{knot::Knot, Input};

pub fn main(input: &Input) -> Result<usize> {
    let mut head_tail = (Knot::new(), Knot::new());

    for (direction, steps) in input.to_owned() {
        for _ in 0..steps {
            head_tail.0.goto(&direction);
            head_tail.1.maybe_move(&head_tail.0);
        }
    }

    Ok(head_tail.1.visited())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 12);
        Ok(())
    }
}
