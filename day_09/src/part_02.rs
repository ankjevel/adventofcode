use std::{collections::LinkedList, io::Result};

use crate::{knot::Knot, Input};

fn handle_tail(head: &mut Knot, tails: &mut LinkedList<Knot>) {
    let mut iter = tails.iter_mut();
    let mut last: Option<&Knot> = None;
    loop {
        let current = iter.next();
        if current.is_none() {
            break;
        }

        let current = current.unwrap();

        if last.is_none() {
            current.maybe_move(&head);
        } else {
            current.maybe_move(last.unwrap());
        };

        last = Some(current);
    }
}

pub fn main(input: &Input) -> Result<usize> {
    let mut head = Knot::new();
    let mut tails: LinkedList<Knot> = LinkedList::from_iter((1..10).map(|_| Knot::new()));

    for (direction, steps) in input.to_owned() {
        for _ in 0..steps {
            head.goto(&direction);
            handle_tail(&mut head, &mut tails);
        }
    }

    Ok(tails.pop_back().unwrap().visited() + 1)
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
