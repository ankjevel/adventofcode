use std::io::Result;

use crate::{knot::Knot, Input};

#[cfg(debug_assertions)]
use crate::print::{combine_head_and_tails, print_grid};

fn handle_tail(head: &mut Knot, tail: &mut Vec<Knot>) {
    let mut last = head;
    for current in tail.iter_mut() {
        current.maybe_move(last);
        last = current;
    }
}

pub fn main(input: &Input) -> Result<usize> {
    let mut head = Knot::new();
    let mut tail = (0..=8).map(|_| Knot::new()).collect::<_>();

    for (direction, steps) in input.to_owned() {
        for _ in 0..steps {
            head.goto(&direction);
            handle_tail(&mut head, &mut tail);
        }
    }

    let last = &tail[8];

    #[cfg(debug_assertions)]
    print_grid(&combine_head_and_tails(&head, &tail), &last.visited);

    Ok(last.visited() + 1)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(
            main(&parse_input(
                &"
                    R 4
                    U 4
                    L 3
                    D 1
                    R 4
                    D 1
                    L 5
                    R 2
                "
            ))?,
            1
        );

        assert_eq!(
            main(&parse_input(
                &"
                    R 5
                    U 8
                    L 8
                    D 3
                    R 17
                    D 10
                    L 25
                    U 20
                "
            ))?,
            36
        );
        Ok(())
    }
}
