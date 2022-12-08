use std::{io::Result, ops::RangeInclusive};

use crate::{max, Input};

pub fn main(input: &Input) -> Result<u32> {
    let (max_x, max_y) = max(input);

    let iter_x = |range: RangeInclusive<usize>, pos_y: &usize, size: &u32| {
        range
            .into_iter()
            .all(|x| input.get(&(x, *pos_y)).unwrap_or(&9) < &size)
    };

    let iter_y = |range: RangeInclusive<usize>, pos_x: &usize, size: &u32| {
        range
            .into_iter()
            .all(|y| input.get(&(*pos_x, y)).unwrap_or(&9) < &size)
    };

    Ok(input
        .clone()
        .into_iter()
        .fold(0, |count, ((pos_x, pos_y), size)| {
            if pos_x == 0 || pos_x == max_x || pos_y == 0 || pos_y == max_y {
                return count + 1;
            }
            if iter_x((pos_x + 1)..=max_x, &pos_y, &size) {
                return count + 1;
            }
            if iter_x(0..=(pos_x - 1), &pos_y, &size) {
                return count + 1;
            }
            if iter_y((pos_y + 1)..=max_y, &pos_x, &size) {
                return count + 1;
            }
            if iter_y(0..=(pos_y - 1), &pos_x, &size) {
                return count + 1;
            }
            return count;
        }))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        30373
        25512
        65332
        33549
        35390
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 21);
        Ok(())
    }
}
