use std::{
    collections::{HashMap, HashSet},
    io::Result,
};

use crate::Input;

pub fn main(input: &Input) -> Result<usize> {
    let mut x_es: Vec<_> = vec![];

    let all: HashMap<_, _> = HashMap::from_iter(input.iter().enumerate().flat_map(|(y, row)| {
        row.chars()
            .enumerate()
            .map(|(x, char)| {
                let pos = (x as isize, y as isize);

                if char == 'X' {
                    x_es.push(pos.to_owned())
                }

                (pos, char)
            })
            .collect::<Vec<_>>()
    }));

    let look = |(x, y): _, v: _, h: _| -> Option<_> {
        let m = (x + v, y + h);
        let a = (x + (v * 2), y + (h * 2));
        let s = (x + (v * 3), y + (h * 3));
        let x = (x, y);

        if all.get(&m) == Some(&'M') && all.get(&a) == Some(&'A') && all.get(&s) == Some(&'S') {
            Some((x, m, a, s))
        } else {
            None
        }
    };

    let found = x_es
        .iter()
        .flat_map(|x_y| {
            [
                (-1, 0),  // left
                (1, 0),   // right
                (0, 1),   // down
                (0, -1),  // up
                (-1, 1),  // down-left
                (1, 1),   // down-right
                (-1, -1), // up-left
                (1, -1),  // up-right
            ]
            .iter()
            .filter_map(|(v, h)| look(*x_y, *v, *h))
            .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();

    Ok(found.len())
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
        assert_eq!(main(&parse_input(EXAMPLE_DATA))?, 18);
        Ok(())
    }
}
