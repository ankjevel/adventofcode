use std::{cmp::Ordering, io::Result};

use crate::Input;

pub fn main((order, pages): &Input) -> Result<usize> {
    Ok(pages
        .iter()
        .filter_map(|pages| {
            let orders = order
                .iter()
                .filter(|(l, r)| pages.contains(l) && pages.contains(r))
                .collect::<Vec<_>>();
            let mut ordered = pages.to_owned().to_owned();
            ordered.sort_by(|a, b| {
                if let Some(order) = orders
                    .iter()
                    .find(|(l, r)| (l == a && r == b) || (l == b && r == a))
                {
                    if order.0 == *a {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Equal
                }
            });

            if ordered.eq(pages) {
                None
            } else {
                Some(ordered.to_owned())
            }
        })
        .fold(0, |sum, pages| {
            let middle = (pages.len() as i8 / 2).abs();
            sum + *pages.get(middle as usize).unwrap() as usize
        }))
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../input/day_05_example");

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 123);
        Ok(())
    }
}
