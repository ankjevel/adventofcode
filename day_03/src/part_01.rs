use std::{io::Result, ops::RangeInclusive};

use crate::{Input, Row};

#[derive(Debug, Clone, Eq, PartialEq)]
struct IteratedRow {
    to_check: Vec<Vec<(usize, usize)>>,
    row: Row,
}

fn around(
    range: &RangeInclusive<usize>,
    max_x: usize,
    max_y: usize,
    row: usize,
) -> Vec<(usize, usize)> {
    let start = range.start().to_owned();
    let end = range.end().to_owned();

    let start_x = if start != 0 { start - 1 } else { start };
    let end_x = if end != max_x { end + 1 } else { end };

    let start_y = if row != 0 { row - 1 } else { row };
    let end_y = if row != max_y { row + 1 } else { row };

    let mut values = vec![];
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if y == row && range.contains(&x) {
                continue;
            }
            values.push((x.to_owned(), y.to_owned()));
        }
    }

    values
}

pub fn main(input: &Input) -> Result<u32> {
    Ok(input
        .rows
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, row)| {
            let to_check = row
                .numbers
                .iter()
                .map(|(range, _)| around(range, input.max_x, input.max_y, index))
                .collect();
            IteratedRow { row, to_check }
        })
        .fold(vec![], |mut matched: Vec<u32>, iterated_row| {
            for (i, range) in iterated_row.to_check.into_iter().enumerate() {
                let exists = range.into_iter().any(|(x, y)| {
                    let rows = input.rows.clone();
                    let y_row = rows.get(y);
                    if y_row.is_none() {
                        return false;
                    }

                    let y_row = y_row.unwrap();
                    let x_numbers = y_row.numbers.clone().into_iter().find(|n| n.0.contains(&x));
                    let x_symbols = y_row.symbols.clone().into_iter().find(|n| n.0 == x);

                    if x_numbers.is_some() || x_symbols.is_some() {
                        return true;
                    }

                    false
                });

                if exists {
                    matched.push(iterated_row.row.numbers.get(i).unwrap().1);
                }
            }

            matched
        })
        .into_iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &'static str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(&EXAMPLE_DATA))?, 4361);
        Ok(())
    }
}
