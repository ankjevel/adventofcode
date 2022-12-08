use std::io::Result;

use crate::{max, Input};

pub fn main(input: &Input) -> Result<u32> {
    let (max_x, max_y) = max(input);

    let not_valid = |x: &usize, y: &usize, size: &u32| input.get(&(*x, *y)).unwrap_or(&9) >= size;

    let iter_x = |range: Box<dyn Iterator<Item = usize>>, pos_y: &usize, size: &u32| {
        let mut n = 0;
        for x in range {
            n = n + 1;
            if not_valid(&x, pos_y, size) {
                break;
            }
        }
        n
    };

    let iter_y = |range: Box<dyn Iterator<Item = usize>>, pos_x: &usize, size: &u32| {
        let mut n = 0;
        for y in range {
            n = n + 1;
            if not_valid(pos_x, &y, size) {
                break;
            }
        }
        n
    };

    Ok(input
        .clone()
        .into_iter()
        .fold(0, |count, ((pos_x, pos_y), size)| {
            if pos_x == 0 || pos_x == max_x || pos_y == 0 || pos_y == max_y {
                return count;
            }

            let right = iter_x(Box::new((pos_x + 1)..=max_x), &pos_y, &size);
            let left = iter_x(Box::new((0..=(pos_x - 1)).rev()), &pos_y, &size);
            let bottom = iter_y(Box::new((pos_y + 1)..=max_y), &pos_x, &size);
            let top = iter_y(Box::new((0..=(pos_y - 1)).rev()), &pos_x, &size);

            let current = left * right * top * bottom;

            if current > count {
                current
            } else {
                count
            }
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
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 8);
        Ok(())
    }
}
