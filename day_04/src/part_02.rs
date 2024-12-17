use std::{collections::HashMap, io::Result};

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    let mut every_a: Vec<_> = vec![];

    let all: HashMap<_, _> = HashMap::from_iter(input.iter().enumerate().flat_map(|(y, row)| {
        row.chars()
            .enumerate()
            .map(|(x, char)| {
                let pos = (x as isize, y as isize);

                if char == 'A' {
                    every_a.push(pos.to_owned())
                }

                (pos, char)
            })
            .collect::<Vec<_>>()
    }));

    let is_ok =
        |a: &char, b: &char| -> bool { (*a == 'M' && *b == 'S') || (*a == 'S' && *b == 'M') };

    let xmas = every_a
        .iter()
        .filter_map(|(x, y)| {
            let top_left = (x - 1, y - 1);
            let top_right = (x + 1, y - 1);
            let bottom_left = (x - 1, y + 1);
            let bottom_right = (x + 1, y + 1);

            if all.contains_key(&top_left)
                && all.contains_key(&top_right)
                && all.contains_key(&bottom_left)
                && all.contains_key(&bottom_right)
            {
                Some([top_left, top_right, (*x, *y), bottom_left, bottom_right])
            } else {
                None
            }
        })
        .filter_map(|positions| {
            let [top_left, top_right, a, bottom_left, bottom_right] =
                positions.map(|pos| (pos, all.get(&pos).unwrap()));

            let ok = is_ok(top_left.1, bottom_right.1) && is_ok(top_right.1, bottom_left.1);

            if ok {
                Some([top_left, top_right, a, bottom_left, bottom_right])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    #[cfg(test)]
    {
        let mut field = vec![vec!['.'; input.first().unwrap().len()]; input.len()];
        for col in &xmas {
            for ((x, y), c) in col {
                field[*y as usize][*x as usize] = **c;
            }
        }

        for y in field {
            print!("\n{}", y.into_iter().collect::<String>());
        }
    }

    Ok(xmas.len())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    const EXAMPLE_DATA: &str = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    ";

    #[test]
    fn it_gets_the_example_correct() -> Result<()> {
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 9);
        Ok(())
    }
}
